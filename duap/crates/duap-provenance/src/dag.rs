//! The derivation graph and rights propagation.
//!
//! STATUS: PRODUCTION (graph and propagation); the *weights* are declared
//! inputs, and where they come from is a separate question answered in
//! `AI_ATTRIBUTION.md`.
//!
//! # What the graph is
//!
//! A directed acyclic graph whose nodes are content-addressed objects and
//! whose edges say "this object was derived from that one, with this
//! attribution weight". Because node identifiers are digests of the objects
//! they name, two organisations that hold the same derived object agree on
//! its identifier without coordinating, and the graph joins across
//! organisational boundaries.
//!
//! # What propagation computes
//!
//! Given a node, [`ProvenanceGraph::attribution`] returns the share of that
//! node attributable to each data subject, as an exact rational. The
//! computation is a weighted sum over all paths, so a subject contributing
//! through two routes is counted once per route.
//!
//! # What propagation does *not* establish
//!
//! The weights are *assertions by whoever recorded the edge*. Three distinct
//! claims are routinely conflated, and DUAP keeps them apart:
//!
//! 1. **Inclusion** -- datum `d` was an input to object `O`. Provable, by a
//!    dataset commitment plus an inclusion proof.
//! 2. **Influence** -- `O` would have been materially different without `d`.
//!    Estimable (leave-one-out, influence functions, Shapley), expensive, and
//!    model-dependent. Never proved by the graph alone.
//! 3. **Economic contribution** -- how much of the value of `O` is owed to
//!    `d`. A matter of agreement or law, informed by (2) but not determined
//!    by it.
//!
//! The graph carries (1) with cryptographic strength. It carries a *number*
//! for (2) and (3) whose provenance is recorded in [`WeightBasis`], so a
//! reader can always see whether a share was measured, agreed, or assumed
//! uniform.
//!
//! # Termination
//!
//! Attribution must stop somewhere or every statistic in the world traces
//! back to everyone. [`DerivationPolicy`] terminates on depth, on negligible
//! share, on node kinds declared terminal (a differentially private release
//! with a small enough epsilon), and on an explicit severing edge. The
//! defaults are conservative and are parameters, not truths; the rationale is
//! in `ECONOMIC_MODEL.md` section "Where attribution stops".

use duap_model::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// What kind of object a node names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    /// Data collected directly about a subject.
    Source,
    /// A transformed or enriched object.
    Derived,
    /// A collection assembled for a purpose.
    Dataset,
    /// Model parameters.
    Model,
    /// A model output or inference.
    Output,
    /// An aggregate statistic.
    Aggregate,
    /// A differentially private release.
    DpRelease,
    /// A synthetic record generated from other data.
    Synthetic,
}

/// Where an edge weight came from. Recorded so that a share can never be
/// read as more authoritative than it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeightBasis {
    /// Every input weighted equally. The default; assumes nothing and
    /// measures nothing.
    Uniform,
    /// Proportional to a declared volume (records, tokens, bytes).
    Volumetric,
    /// Agreed bilaterally or set by a schedule.
    Contractual,
    /// Estimated by an attribution method. The method and its parameters
    /// belong in the edge's `note`.
    Measured,
    /// Set by a regulator or dispute resolution.
    Adjudicated,
}

/// An edge from an input object to a derived one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    #[serde(rename = "f")]
    pub from: ContentId,
    /// Share of the derived object attributed to this input, in `[0, 1]`.
    /// Weights on a node's incoming edges should sum to 1; the graph reports
    /// it when they do not rather than silently normalising.
    #[serde(rename = "w")]
    pub weight: Ratio,
    #[serde(rename = "b")]
    pub basis: WeightBasis,
    /// Severing edge: provenance is recorded for audit but attribution does
    /// not flow through it. Used where a transformation is agreed or
    /// adjudicated to extinguish the upstream claim.
    #[serde(rename = "sv", default, skip_serializing_if = "is_false")]
    pub severed: bool,
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl Edge {
    pub fn uniform(from: ContentId, weight: Ratio) -> Edge {
        Edge {
            from,
            weight,
            basis: WeightBasis::Uniform,
            severed: false,
            note: None,
        }
    }
}

/// A node in the derivation graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvNode {
    #[serde(rename = "id")]
    pub id: ContentId,
    #[serde(rename = "k")]
    pub kind: NodeKind,
    #[serde(rename = "ct")]
    pub controller: OrgId,
    #[serde(rename = "at")]
    pub created_at: Timestamp,
    #[serde(rename = "in", default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<Edge>,
    #[serde(rename = "op", default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// The subject a source node is about. Only `Source` nodes carry one.
    #[serde(rename = "sb", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<SubjectRef>,
    /// Epsilon spent, for DP releases, in micro-units.
    #[serde(rename = "ep", default, skip_serializing_if = "Option::is_none")]
    pub epsilon_micro: Option<u64>,
}

impl ProvNode {
    pub fn source(id: ContentId, controller: OrgId, subject: SubjectRef, at: Timestamp) -> ProvNode {
        ProvNode {
            id,
            kind: NodeKind::Source,
            controller,
            created_at: at,
            inputs: Vec::new(),
            operation: None,
            subject: Some(subject),
            epsilon_micro: None,
        }
    }

    pub fn derived(
        id: ContentId,
        kind: NodeKind,
        controller: OrgId,
        at: Timestamp,
        inputs: Vec<Edge>,
        operation: Operation,
    ) -> ProvNode {
        ProvNode {
            id,
            kind,
            controller,
            created_at: at,
            inputs,
            operation: Some(operation),
            subject: None,
            epsilon_micro: None,
        }
    }
}

/// Rules that bound attribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DerivationPolicy {
    /// Maximum number of derivation hops traversed.
    pub max_depth: u8,
    /// Shares strictly below this are dropped. Keeps the computation finite
    /// and stops rounding dust from producing millions of nil claims.
    pub min_share: Ratio,
    /// Node kinds at which attribution stops.
    pub terminal_kinds: Vec<NodeKind>,
    /// A DP release with epsilon at or below this (micro-units) terminates
    /// attribution. `None` means DP releases never terminate on that ground.
    ///
    /// This is a *policy* threshold. Differential privacy bounds what can be
    /// learned about an individual; it does not by itself extinguish an
    /// economic claim, and DUAP does not assert that it does.
    pub dp_terminates_at_or_below_micro: Option<u64>,
}

impl Default for DerivationPolicy {
    fn default() -> Self {
        DerivationPolicy {
            max_depth: 16,
            min_share: Ratio::new(1, 1_000_000).expect("non-zero denominator"),
            terminal_kinds: vec![],
            dp_terminates_at_or_below_micro: None,
        }
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum GraphError {
    #[error("node {0} is not in the graph")]
    Unknown(String),
    #[error("node {0} already exists with different content")]
    Conflict(String),
    #[error("adding {node} would create a cycle through {through}")]
    Cycle { node: String, through: String },
    #[error("node {node} has input weights summing to {sum}, not 1")]
    WeightsDoNotSum { node: String, sum: String },
    #[error("attribution exceeded the policy depth limit of {0} hops")]
    DepthExceeded(u8),
    #[error("{0}")]
    Model(#[from] ModelError),
}

/// A derivation graph.
#[derive(Debug, Clone, Default)]
pub struct ProvenanceGraph {
    nodes: BTreeMap<String, ProvNode>,
    children: BTreeMap<String, BTreeSet<String>>,
}

impl ProvenanceGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn get(&self, id: &ContentId) -> Option<&ProvNode> {
        self.nodes.get(&id.to_string())
    }

    pub fn nodes(&self) -> impl Iterator<Item = &ProvNode> {
        self.nodes.values()
    }

    /// Insert a node.
    ///
    /// Inputs must already be present: a graph that admits forward references
    /// cannot be checked for cycles at insertion time, and a cycle in a
    /// provenance graph is either a bug or an attempt to inflate attribution
    /// by looping value back on itself.
    pub fn insert(&mut self, node: ProvNode) -> Result<(), GraphError> {
        let key = node.id.to_string();
        if let Some(existing) = self.nodes.get(&key) {
            if *existing != node {
                return Err(GraphError::Conflict(key));
            }
            return Ok(());
        }
        for e in &node.inputs {
            let ik = e.from.to_string();
            // Self-reference first: it is a cycle, not a missing node, and
            // saying so makes the failure legible.
            if ik == key {
                return Err(GraphError::Cycle {
                    node: key.clone(),
                    through: ik,
                });
            }
            if !self.nodes.contains_key(&ik) {
                return Err(GraphError::Unknown(ik));
            }
        }
        // Inputs all exist and none is this node, so no cycle can be created:
        // every edge points into the already-acyclic set. Verified by the
        // `insertion_order_prevents_cycles` property test.
        for e in &node.inputs {
            self.children
                .entry(e.from.to_string())
                .or_default()
                .insert(key.clone());
        }
        self.nodes.insert(key, node);
        Ok(())
    }

    /// Check that a node's input weights sum to exactly 1.
    pub fn check_weights(&self, id: &ContentId) -> Result<(), GraphError> {
        let n = self
            .get(id)
            .ok_or_else(|| GraphError::Unknown(id.to_string()))?;
        if n.inputs.is_empty() {
            return Ok(());
        }
        let mut sum = Ratio::ZERO;
        for e in &n.inputs {
            sum = sum.add(e.weight)?;
        }
        if sum != Ratio::ONE {
            return Err(GraphError::WeightsDoNotSum {
                node: id.to_string(),
                sum: sum.to_string(),
            });
        }
        Ok(())
    }

    /// Depth of a node: 0 for a source, else 1 + the maximum input depth.
    pub fn depth(&self, id: &ContentId) -> Result<u8, GraphError> {
        let mut memo: BTreeMap<String, u8> = BTreeMap::new();
        self.depth_inner(id, &mut memo, 0)
    }

    fn depth_inner(
        &self,
        id: &ContentId,
        memo: &mut BTreeMap<String, u8>,
        guard: u16,
    ) -> Result<u8, GraphError> {
        if guard > 1024 {
            return Err(GraphError::DepthExceeded(255));
        }
        let key = id.to_string();
        if let Some(d) = memo.get(&key) {
            return Ok(*d);
        }
        let n = self
            .get(id)
            .ok_or_else(|| GraphError::Unknown(key.clone()))?;
        let mut d = 0u8;
        for e in &n.inputs {
            let di = self.depth_inner(&e.from, memo, guard + 1)?.saturating_add(1);
            d = d.max(di);
        }
        memo.insert(key, d);
        Ok(d)
    }

    /// Objects derived directly from `id`.
    pub fn children_of(&self, id: &ContentId) -> Vec<&ProvNode> {
        self.children
            .get(&id.to_string())
            .map(|s| s.iter().filter_map(|k| self.nodes.get(k)).collect())
            .unwrap_or_default()
    }

    /// Every object reachable downstream of `id`, breadth first.
    pub fn descendants(&self, id: &ContentId) -> Vec<&ProvNode> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(id.to_string());
        while let Some(k) = q.pop_front() {
            if let Some(cs) = self.children.get(&k) {
                for c in cs {
                    if seen.insert(c.clone()) {
                        if let Some(n) = self.nodes.get(c) {
                            out.push(n);
                        }
                        q.push_back(c.clone());
                    }
                }
            }
        }
        out
    }

    /// Attribution shares for `id`, per data subject.
    ///
    /// Returns exact rationals. A subject reachable by several paths has the
    /// path products summed. Shares below `policy.min_share` are dropped, so
    /// the result generally sums to slightly less than 1; the shortfall is
    /// returned as the second element so that the caller can account for it
    /// rather than silently losing it.
    pub fn attribution(
        &self,
        id: &ContentId,
        policy: &DerivationPolicy,
    ) -> Result<(BTreeMap<SubjectRef, Ratio>, Ratio), GraphError> {
        let mut out: BTreeMap<SubjectRef, Ratio> = BTreeMap::new();
        let mut dropped = Ratio::ZERO;
        self.walk(id, Ratio::ONE, 0, policy, &mut out, &mut dropped)?;
        Ok((out, dropped))
    }

    #[allow(clippy::only_used_in_recursion)]
    fn walk(
        &self,
        id: &ContentId,
        share: Ratio,
        depth: u8,
        policy: &DerivationPolicy,
        out: &mut BTreeMap<SubjectRef, Ratio>,
        dropped: &mut Ratio,
    ) -> Result<(), GraphError> {
        if share.num == 0 {
            return Ok(());
        }
        // Compare share < min_share as exact cross-multiplication.
        if (share.num as i128) * (policy.min_share.den as i128)
            < (policy.min_share.num as i128) * (share.den as i128)
        {
            *dropped = dropped.add(share)?;
            return Ok(());
        }
        if depth > policy.max_depth {
            return Err(GraphError::DepthExceeded(policy.max_depth));
        }
        let n = self
            .get(id)
            .ok_or_else(|| GraphError::Unknown(id.to_string()))?;

        // Terminal by kind.
        if policy.terminal_kinds.contains(&n.kind) {
            *dropped = dropped.add(share)?;
            return Ok(());
        }
        // Terminal by differential privacy budget.
        if n.kind == NodeKind::DpRelease {
            if let (Some(limit), Some(eps)) = (policy.dp_terminates_at_or_below_micro, n.epsilon_micro)
            {
                if eps <= limit {
                    *dropped = dropped.add(share)?;
                    return Ok(());
                }
            }
        }

        if let Some(s) = n.subject {
            let e = out.entry(s).or_insert(Ratio::ZERO);
            *e = e.add(share)?;
            return Ok(());
        }

        for edge in &n.inputs {
            if edge.severed {
                *dropped = dropped.add(share.mul(edge.weight)?)?;
                continue;
            }
            let next = share.mul(edge.weight)?;
            self.walk(&edge.from, next, depth + 1, policy, out, dropped)?;
        }
        Ok(())
    }

    /// Every path from a subject's source nodes to `id`, for explanation in a
    /// dashboard or a dispute. Bounded by `max_paths` to keep the output
    /// finite on wide graphs.
    pub fn paths_from_subject(
        &self,
        id: &ContentId,
        subject: SubjectRef,
        max_paths: usize,
    ) -> Vec<Vec<ContentId>> {
        let mut out = Vec::new();
        let mut stack = vec![*id];
        self.collect_paths(id, subject, &mut stack, &mut out, max_paths, 0);
        for p in out.iter_mut() {
            p.reverse();
        }
        out
    }

    fn collect_paths(
        &self,
        id: &ContentId,
        subject: SubjectRef,
        stack: &mut Vec<ContentId>,
        out: &mut Vec<Vec<ContentId>>,
        max_paths: usize,
        depth: u16,
    ) {
        if out.len() >= max_paths || depth > 256 {
            return;
        }
        let Some(n) = self.get(id) else { return };
        if n.subject == Some(subject) {
            out.push(stack.clone());
            return;
        }
        for e in &n.inputs {
            stack.push(e.from);
            self.collect_paths(&e.from, subject, stack, out, max_paths, depth + 1);
            stack.pop();
        }
    }
}
