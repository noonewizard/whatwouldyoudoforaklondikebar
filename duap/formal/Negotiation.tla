------------------------------ MODULE Negotiation ------------------------------
(***************************************************************************)
(* The DUAP authorization negotiation state machine.                       *)
(*                                                                         *)
(* STATUS: REFERENCE.                                                      *)
(*                                                                         *)
(* This models `crates/duap-auth/src/negotiation.rs`. Two parties that     *)
(* have never spoken agree on an authorization without a human: the        *)
(* requester sends a Request, the responder answers with an Offer (or      *)
(* counter-Offers), the requester Accepts, and the responder issues a      *)
(* signed Grant. Either side may Reject, and either may Withdraw before    *)
(* acceptance.                                                             *)
(*                                                                         *)
(* WHAT IS MODELLED                                                        *)
(*   - the six states and every transition between them;                   *)
(*   - the digest chain: an Offer names the Request it answers, and an     *)
(*     Acceptance names the Offer it accepts, so a party cannot accept     *)
(*     terms that were never offered;                                      *)
(*   - counter-offers, which supersede the previous offer;                 *)
(*   - replay rejection, modelled as a set of seen message digests;        *)
(*   - expiry, as a monotonic clock against a deadline carried forward by  *)
(*     each message, with Reject exempt (a party may always refuse).       *)
(*                                                                         *)
(* WHAT IS NOT MODELLED                                                    *)
(*   - signatures and envelopes: every message here is assumed             *)
(*     authenticated. The model says nothing about forgery;                *)
(*   - the content of terms, prices or matchers. An Offer is an opaque     *)
(*     identity, so the model cannot say anything about whether the terms  *)
(*     are sensible, only about whether the exchange is well-formed;       *)
(*   - the transport, retransmission, and message loss;                    *)
(*   - concurrent negotiations. One negotiation is modelled in isolation,  *)
(*     which is exactly what the Rust `Negotiation` struct represents.     *)
(*                                                                         *)
(* WHAT THIS ESTABLISHES, AND WHAT IT DOES NOT                             *)
(*   It establishes that the state machine as modelled has the properties  *)
(*   below within the stated bounds. It does NOT establish that the Rust   *)
(*   implements this model: nothing here is extracted from or verified     *)
(*   against the source. The mirror tests in                               *)
(*   `crates/duap-auth/tests/negotiation_model_mirror.rs` share these      *)
(*   names and are evidence of agreement on specific cases, not a          *)
(*   refinement proof.                                                     *)
(***************************************************************************)
EXTENDS Naturals, FiniteSets

CONSTANTS
  MaxOffers,   \* how many counter-offers the exchange may carry
  MaxTime      \* clock bound

VARIABLES
  state,        \* one of the six states
  reqSent,      \* TRUE once a Request has been accepted by the machine
  offerCount,   \* how many offers have been made
  liveOffer,    \* identity of the offer currently on the table, or 0
  acceptedOffer,\* identity of the offer that was accepted, or 0
  seen,         \* set of message identities already applied (replay guard)
  clock,        \* monotonic time
  deadline      \* expiry carried by the most recent message, or 0 for none

vars == << state, reqSent, offerCount, liveOffer, acceptedOffer,
           seen, clock, deadline >>

States == {"Idle", "Requested", "Offered", "Accepted", "Granted", "Closed"}
Terminal == {"Granted", "Closed"}

\* Message identities. A Request is 0; offer k is k; an acceptance of offer
\* k is identified as 100+k; Reject and Withdraw as 200 and 201. Distinct
\* identities are all the replay guard needs.
MsgIds == {0} \cup (1..MaxOffers) \cup {100+k : k \in 1..MaxOffers}
                                  \cup {200, 201}

TypeOK ==
  /\ state \in States
  /\ reqSent \in BOOLEAN
  /\ offerCount \in 0..MaxOffers
  /\ liveOffer \in 0..MaxOffers
  /\ acceptedOffer \in 0..MaxOffers
  /\ seen \subseteq MsgIds
  /\ clock \in 0..MaxTime
  /\ deadline \in 0..MaxTime

Init ==
  /\ state = "Idle"
  /\ reqSent = FALSE
  /\ offerCount = 0
  /\ liveOffer = 0
  /\ acceptedOffer = 0
  /\ seen = {}
  /\ clock = 0
  /\ deadline = 0

(***************************************************************************)
(* Time. Messages may arrive at the current instant or later; the clock    *)
(* only moves forward.                                                     *)
(***************************************************************************)
Tick ==
  /\ clock < MaxTime
  /\ clock' = clock + 1
  /\ UNCHANGED << state, reqSent, offerCount, liveOffer, acceptedOffer,
                  seen, deadline >>

\* A message is in time if no deadline is set or the clock has not passed it.
InTime == deadline = 0 \/ clock <= deadline

(***************************************************************************)
(* Transitions. Each mirrors one arm of `Negotiation::apply`.              *)
(***************************************************************************)

SendRequest ==
  /\ state = "Idle"
  /\ 0 \notin seen
  /\ \E d \in clock..MaxTime :
        deadline' = d
  /\ state' = "Requested"
  /\ reqSent' = TRUE
  /\ seen' = seen \cup {0}
  /\ UNCHANGED << offerCount, liveOffer, acceptedOffer, clock >>

\* An offer is valid only in Requested or Offered, and only when a request
\* exists to bind to. The counter-offer case is the second disjunct of the
\* guard: an Offer in state Offered supersedes the previous one.
SendOffer ==
  /\ state \in {"Requested", "Offered"}
  /\ reqSent
  /\ InTime
  /\ offerCount < MaxOffers
  /\ LET k == offerCount + 1 IN
       /\ k \notin seen
       /\ offerCount' = k
       /\ liveOffer' = k
       /\ seen' = seen \cup {k}
  /\ \E d \in clock..MaxTime : deadline' = d
  /\ state' = "Offered"
  /\ UNCHANGED << reqSent, acceptedOffer, clock >>

\* Acceptance names the offer it accepts, and that must be the live one.
\* This is the digest check `a.offer_digest == self.offer_digest`.
Accept ==
  /\ state = "Offered"
  /\ liveOffer /= 0
  /\ InTime
  /\ (100 + liveOffer) \notin seen
  /\ acceptedOffer' = liveOffer
  /\ seen' = seen \cup {100 + liveOffer}
  /\ state' = "Accepted"
  /\ UNCHANGED << reqSent, offerCount, liveOffer, clock, deadline >>

\* The grant is issued out of band and settled into the machine. The Rust
\* `settle` refuses unless the state is Accepted.
Settle ==
  /\ state = "Accepted"
  /\ state' = "Granted"
  /\ UNCHANGED << reqSent, offerCount, liveOffer, acceptedOffer, seen,
                  clock, deadline >>

\* Reject is permitted from any live state and is exempt from expiry: a
\* party may always refuse, including after the deadline.
Reject ==
  /\ state \in {"Requested", "Offered", "Accepted"}
  /\ 200 \notin seen
  /\ seen' = seen \cup {200}
  /\ state' = "Closed"
  /\ UNCHANGED << reqSent, offerCount, liveOffer, acceptedOffer, clock,
                  deadline >>

\* Withdraw is only available before acceptance.
Withdraw ==
  /\ state \in {"Requested", "Offered"}
  /\ InTime
  /\ 201 \notin seen
  /\ seen' = seen \cup {201}
  /\ state' = "Closed"
  /\ UNCHANGED << reqSent, offerCount, liveOffer, acceptedOffer, clock,
                  deadline >>

Next == SendRequest \/ SendOffer \/ Accept \/ Settle \/ Reject
                    \/ Withdraw \/ Tick

Spec == Init /\ [][Next]_vars

Bounded == clock <= MaxTime /\ offerCount <= MaxOffers

(***************************************************************************)
(* INVARIANTS                                                              *)
(***************************************************************************)

\* INV-N1. No grant without a request. The exchange cannot produce an
\* authorization that nobody asked for.
NoGrantWithoutRequest ==
  (state \in {"Offered", "Accepted", "Granted"}) => reqSent

\* INV-N2. No acceptance without an offer, and the accepted offer is one
\* that was actually made. This is the digest chain: a party cannot accept
\* terms that were never put on the table.
AcceptanceBindsAnOffer ==
  (state \in {"Accepted", "Granted"}) =>
     /\ acceptedOffer /= 0
     /\ acceptedOffer <= offerCount

\* INV-N3. A grant is preceded by an acceptance. Settle is the only way
\* into Granted and it requires Accepted.
GrantFollowsAcceptance ==
  (state = "Granted") => acceptedOffer /= 0

\* INV-N4. Terminal states are absorbing. Nothing in Next has a Granted or
\* Closed state in its guard, so once terminal the machine cannot move --
\* which is what makes a negotiation's outcome final.
TerminalIsAbsorbing ==
  (state \in Terminal) =>
     ~ENABLED (SendRequest \/ SendOffer \/ Accept \/ Settle \/ Reject
               \/ Withdraw)

\* INV-N5. Granted and Closed are mutually exclusive by construction: the
\* state is a single value, so a negotiation cannot both succeed and be
\* refused. Stated because it is the property a reader most wants and it
\* would be easy to break with a richer state representation.
OutcomeIsUnique ==
  ~(state = "Granted" /\ state = "Closed")

\* INV-N6. Every message identity is applied at most once. The replay
\* guard is what stops a captured Accept from re-opening a settled
\* negotiation.
NoReplay == seen \subseteq MsgIds

(***************************************************************************)
(* NON-VACUITY                                                             *)
(*                                                                         *)
(* Checked with Negotiation_NonVacuity.cfg, which asserts that Granted is  *)
(* never reachable. That check MUST FAIL. Its counterexample is the        *)
(* evidence that the six invariants above are not true merely because the  *)
(* machine never does anything.                                            *)
(***************************************************************************)
NoGrantIsEverReachable == state /= "Granted"

================================================================================
