---------------------------- MODULE Authorization ----------------------------
(***************************************************************************)
(* A TLA+ model of DUAP's authorization semantics.                         *)
(*                                                                         *)
(* STATUS: REFERENCE.  The model checks the combining algorithm and the    *)
(* revocation semantics, not the implementation.  What it abstracts is     *)
(* recorded in formal/README.md: matchers are explicit class and purpose   *)
(* sets rather than the full selector language, and obligations are not    *)
(* modelled.  Each invariant below is mirrored by a Rust test of the same  *)
(* name, so model and code cannot drift silently.                          *)
(*                                                                         *)
(* The invariants quantify over *every decision computable in the current  *)
(* state*, rather than over a log of past decisions.  That is both         *)
(* stronger and far cheaper to check: a violation in any reachable         *)
(* configuration is caught, and the state space stays small enough to      *)
(* explore exhaustively.                                                   *)
(*                                                                         *)
(*   DenyOverrides           -- INV-A1                                     *)
(*   PermitIsExplained       -- INV-A2                                     *)
(*   RevocationIsProspective -- INV-A4                                     *)
(*   NoEscapeByAmendment     -- INV-A5                                     *)
(***************************************************************************)
EXTENDS Naturals, FiniteSets

CONSTANTS Classes,   \* set of data classes
          Purposes,  \* set of purposes
          MaxEpoch,  \* highest grant epoch explored
          MaxTime    \* highest clock value

VARIABLES epoch,        \* current grant epoch
          history,      \* epoch |-> set of terms in force at that epoch
          revocations,  \* set of revocation records issued so far
          clock         \* discrete time

vars == <<epoch, history, revocations, clock>>

(***************************************************************************)
(* A term is [id, effect, cls, pur].  The pool is small and explicit so    *)
(* that the state space stays exhaustively checkable, and covers the four  *)
(* shapes that matter: a narrow permit, a broad permit, a narrow deny and  *)
(* a broad deny.                                                           *)
(***************************************************************************)
AllTerms ==
  { [id |-> 1, effect |-> "Permit", cls |-> {"c1"},       pur |-> {"p1"}],
    [id |-> 2, effect |-> "Permit", cls |-> {"c1", "c2"}, pur |-> {"p1", "p2"}],
    [id |-> 3, effect |-> "Deny",   cls |-> {"c2"},       pur |-> {"p1", "p2"}],
    [id |-> 4, effect |-> "Deny",   cls |-> {"c1", "c2"}, pur |-> {"p2"}] }

TermIds == {1, 2, 3, 4}

Applicable(ts, c, p)  == { t \in ts : c \in t.cls /\ p \in t.pur }
DenyTerms(ts, c, p)   == { t \in Applicable(ts, c, p) : t.effect = "Deny" }
PermitTerms(ts, c, p) == { t \in Applicable(ts, c, p) : t.effect = "Permit" }

(***************************************************************************)
(* A revocation names the epoch it was issued against, the purposes it     *)
(* withdraws and the time from which it bites.  It binds that epoch and    *)
(* every later one, which is what stops an amendment escaping it.          *)
(***************************************************************************)
Revokes(r, ep, p, tm) == r.epoch =< ep /\ p \in r.purs /\ r.from =< tm

RevokedNow(revs, ep, p, tm) == \E r \in revs : Revokes(r, ep, p, tm)

LivePermits(ts, revs, c, p, ep, tm) ==
  IF RevokedNow(revs, ep, p, tm) THEN {} ELSE PermitTerms(ts, c, p)

(***************************************************************************)
(* Deny overrides; then live permits; otherwise the grant's default, which *)
(* in DUAP is always Deny.                                                 *)
(***************************************************************************)
Effect(ts, revs, c, p, ep, tm) ==
  IF DenyTerms(ts, c, p) # {} THEN "Deny"
  ELSE IF LivePermits(ts, revs, c, p, ep, tm) # {} THEN "Permit"
  ELSE "Deny"

PermittingIds(ts, revs, c, p, ep, tm) ==
  { t.id : t \in LivePermits(ts, revs, c, p, ep, tm) }

Epochs == 1..MaxEpoch
Times  == 1..MaxTime

RevocationRecords ==
  [ epoch : Epochs, purs : (SUBSET Purposes) \ {{}}, from : Times ]

TypeOK ==
  /\ epoch \in Epochs
  /\ history \in [ Epochs -> SUBSET AllTerms ]
  /\ revocations \in SUBSET RevocationRecords
  /\ clock \in Times

Init ==
  /\ epoch = 1
  /\ history = [ e \in Epochs |-> IF e = 1 THEN AllTerms ELSE {} ]
  /\ revocations = {}
  /\ clock = 1

Tick ==
  /\ clock < MaxTime
  /\ clock' = clock + 1
  /\ UNCHANGED <<epoch, history, revocations>>

(***************************************************************************)
(* Amending a grant produces a new epoch with a new term set.  The old     *)
(* epoch's terms stay in history, because an event cites the epoch it      *)
(* relied on and must still be evaluable against it.                       *)
(***************************************************************************)
Amend ==
  /\ epoch < MaxEpoch
  /\ \E ts \in SUBSET AllTerms :
        /\ history' = [ history EXCEPT ![epoch + 1] = ts ]
        /\ epoch' = epoch + 1
  /\ UNCHANGED <<revocations, clock>>

Revoke ==
  /\ \E purs \in (SUBSET Purposes) \ {{}} :
        revocations' = revocations \cup
          { [ epoch |-> epoch, purs |-> purs, from |-> clock ] }
  /\ UNCHANGED <<epoch, history, clock>>

Next == Tick \/ Amend \/ Revoke

Spec == Init /\ [][Next]_vars /\ WF_vars(Next)

(***************************************************************************)
(* The set of decisions evaluable in the current state: any class, any     *)
(* purpose, any issued epoch, any time up to the clock.                    *)
(***************************************************************************)
Evaluable == { <<c, p, ep, tm>> \in Classes \X Purposes \X Epochs \X Times :
                 ep =< epoch /\ tm =< clock }

Eff(q)  == Effect(history[q[3]], revocations, q[1], q[2], q[3], q[4])
Perm(q) == PermittingIds(history[q[3]], revocations, q[1], q[2], q[3], q[4])

(***************************************************************************)
(* INV-A1.  An applicable Deny always wins, whatever else applies and in   *)
(* whatever order the terms appear.                                        *)
(***************************************************************************)
DenyOverrides ==
  \A q \in Evaluable :
    DenyTerms(history[q[3]], q[1], q[2]) # {} => Eff(q) = "Deny"

(***************************************************************************)
(* INV-A2.  A permission always names at least one term that produced it.  *)
(* DUAP grants default to Deny, so there is no unexplained Permit.         *)
(***************************************************************************)
PermitIsExplained ==
  \A q \in Evaluable : Eff(q) = "Permit" => Perm(q) # {}

(***************************************************************************)
(* INV-A4.  A revocation in force at the time of the decision denies.      *)
(***************************************************************************)
RevocationIsProspective ==
  \A q \in Evaluable :
    RevokedNow(revocations, q[3], q[2], q[4]) => Eff(q) = "Deny"

(***************************************************************************)
(* INV-A5.  A revocation issued against epoch n binds every epoch from n   *)
(* onwards, so amending the grant cannot resurrect a revoked permission.   *)
(***************************************************************************)
NoEscapeByAmendment ==
  \A q \in Evaluable :
    (\E r \in revocations :
        r.epoch =< q[3] /\ q[2] \in r.purs /\ r.from =< q[4]) => Eff(q) = "Deny"

(***************************************************************************)
(* Non-vacuity.  Checked as an invariant that is *expected to fail*: if it *)
(* held, every decision would be a Deny and the four invariants above      *)
(* would be trivially true.  `formal/check.sh` asserts that TLC reports a  *)
(* violation of this one, and no violation of the others.                  *)
(***************************************************************************)
NoPermitIsEverReachable == \A q \in Evaluable : Eff(q) # "Permit"

(***************************************************************************)
(* State constraint.  One outstanding revocation already exercises the     *)
(* interaction between scope, epoch and effective time; more only          *)
(* multiplies the state space.                                             *)
(***************************************************************************)
Bounded == Cardinality(revocations) =< 1

=============================================================================
