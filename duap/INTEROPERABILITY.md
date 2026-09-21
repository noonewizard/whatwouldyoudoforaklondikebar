# Interoperability

**Status:** SPECIFIED (designs) / UNIMPLEMENTED (all bindings) · 2026-09-21

**Read this first.** Every binding in this document is a design. **None is
implemented.** `docs/STATUS.md` records the subsystems that would host them
as CONCEPT, and `docs/standards/prior-art.md` §10 lists them as gaps. This
file exists so the designs are written down and their absence is visible,
not to describe capability the repository has.

DUAP's stated position is that it should bind existing standards rather
than reinvent them: three objects defined, nine standards bound. That ratio
is the project's argument for existing, and this document is where it has
to be made good.

## The bindings

### RSL (Really Simple Licensing) — highest priority

**Why first:** RSL 1.0 standardises machine-readable licensing terms
including pay-per-crawl and pay-per-inference, and is the terms layer the
AI content market is actually adopting **[search-summary]**. DUAP's
surviving claim (`docs/reviews/falsification.md` §13) is accounting for
pay-per-inference, a term RSL expresses and nothing measures. Without this
binding the claim is theoretical.

**Design:** derive a DUAP `Grant` from an RSL licence document. The RSL
licence is authoritative for terms; the derived grant is a cached,
canonically encoded projection of it for evaluation, carrying the source
document's digest in `extensions` under a non-reserved key so the
projection is auditable against its source.

**Open question:** RSL is XML and DUAP grants are canonical CBOR. The
projection is lossy in one direction by design — a grant need not represent
every RSL element, only those affecting evaluation and pricing. Which
elements those are has not been enumerated, and that enumeration is the
actual work.

### W3C ODRL 2.2

**Design:** a bidirectional profile mapping `Term` ↔ ODRL
`Permission`/`Prohibition`/`Duty`, `Matcher` ↔ ODRL constraints, and
`PricingRule` ↔ a `Duty` with a payment action.

**Where it will not map cleanly, stated rather than discovered later:**
DUAP's deny-overrides combining algorithm with obligation union
(ADR-0012) is a specific evaluation semantics; ODRL does not mandate one.
A round trip through ODRL and back can therefore preserve the terms and
lose the decision. The profile must say so, and the DUAP grant must remain
authoritative for evaluation wherever one exists.

### IDS Dataspace Protocol

**Design:** a DUAP `Grant` derived from a Dataspace Protocol Agreement, and
a DUAP `Receipt` emitted by a connector as a transfer completes and as use
is subsequently metered.

**Why this is the cleanest fit in the document:** the Dataspace Protocol
covers catalogue, negotiation and transfer initiation and its specification
states it does not address metering, billing, accounting, clearing or
settlement (retrieved 2026-09-21). The two are complementary by
construction rather than by accommodation.

### W3C PROV-O

**Design:** serialise `ProvenanceGraph` as PROV-O — a DUAP event is a
`prov:Activity`, a data resource or derived artefact is a `prov:Entity`,
an agent is a `prov:Agent`, and derivation edges are `prov:wasDerivedFrom`.

**What the serialisation must not lose:** DUAP's edges are
integrity-protected by the event digests they connect. A PROV-O
serialisation is a view, and a consumer that trusts the view without the
digests has weaker evidence than one that does not. The serialiser should
emit the digests as literals so the view is checkable.

### W3C DPV

**Design:** map DUAP's purpose taxonomy to DPV purposes and its
lawful-basis codes to DPV's. `duap-model`'s taxonomy is generated from
`ontology/duap-ontology-v1.json` precisely so this mapping can be produced
mechanically rather than maintained by hand.

**The constraint rule 04 imposes:** a mapped lawful basis still records
only what a controller asserts. Mapping it to a DPV term does not make it
a determination, and the mapping documentation must say so.

### OpenTelemetry — highest leverage

**Why it matters more than its position here suggests:** for an
organisation already emitting OTel, a collector processor turns adoption
from an instrumentation project into a configuration change.
`docs/market/go-to-market.md` §4 calls this the single highest-leverage
unbuilt item in the repository.

**Design:** a collector processor that maps selected spans to DUAP events,
signs them with the deployment's agent key, and exports them to a clearing
node. Span attributes supply resource, operation, quantity and purpose via
a configured mapping.

**The honest limit, which must appear in the processor's own
documentation:** a span is a self-report by the system being measured.
Signing it makes it attributable and non-repudiable. It does not make it
true, and anyone reading "cryptographically signed telemetry" as "verified
usage" has been misled. RISK-02.

### FinOps FOCUS

**Design:** map DUAP obligations to FOCUS columns so that data-rights costs
appear in the same cost reporting as infrastructure costs.

**Why it is worth doing at all:** it puts a data-licensing obligation in
front of the people who already reconcile cloud spend, which is where the
budget and the reconciliation habit already live.

### ISO 20022

**Design:** a cleared DUAP obligation maps to a payment initiation with the
receipt reference in the remittance information, so a payment can be traced
back to the evidence that justified it.

**The boundary this preserves:** DUAP computes what is owed and stops. It
does not move money, hold funds or become a payment system — a constraint
driven by `docs/legal/regulatory-matrix.md` §5 as much as by scope.

### DCAT and identity

DCAT: a DUAP resource identifier should resolve to a DCAT description
where one exists. Identity: DUAP defines an identifier format and
self-certifying key identifiers and nothing else, binding to whatever
OAuth/OIDC/VC/DID infrastructure a deployment has (ADR-0006). It inherits
that layer's weaknesses and says so rather than pretending to solve
identity.

## Is "the economic binding layer across existing information standards" defensible?

The directive asks this directly. The honest answer has three parts.

**Defensible as a design.** The gap analysis holds: no standard reviewed
represents a unit of data usage as an evidence-bearing object carrying an
economic obligation across an organizational boundary, and the one source
this environment could retrieve — the Dataspace Protocol specification —
states the gap explicitly.

**Not yet defensible as a description.** Zero bindings are implemented. A
binding layer that binds nothing is a claim about the future, and the claim
should be written in the future tense until at least two bindings exist and
interoperate.

**Possibly not defensible at all, if the market answers differently.**
`docs/market/competitive-landscape.md` records that the AI content vertical
built a trusted-intermediary meter instead, and it works. A binding layer
is only necessary if evidence has to cross boundaries between parties who
do not share an intermediary. Whether that is the world we get is
unresolved.

## Order of work

1. RSL — without it the surviving claim is theoretical
2. OpenTelemetry — without it adoption cost stays prohibitive
3. Dataspace Protocol — the cleanest complementary fit
4. PROV-O, DPV — evidence quality and privacy semantics
5. FOCUS, ISO 20022, DCAT — reporting and settlement convenience
