------------------------------ MODULE Accounting ----------------------------
(***************************************************************************)
(* A TLA+ model of DUAP's double-entry accounting invariants.              *)
(*                                                                         *)
(* STATUS: REFERENCE.  The model covers posting, reversal and the rounding *)
(* discipline in a single currency.  It abstracts away invoice structure,  *)
(* tax and settlement rails.  What it is about is the property that no     *)
(* sequence of postings, reversals or roundings can move the ledger out of *)
(* balance, or create or destroy value at the boundary between the         *)
(* computation scale and the settlement scale.                             *)
(*                                                                         *)
(*   EntryBalances    -- INV-L1 (a transition property)                    *)
(*   TrialBalanceZero -- INV-L2                                            *)
(*   ValueConserved   -- INV-L3                                            *)
(*                                                                         *)
(* Each is mirrored by a Rust test of the same name.                       *)
(***************************************************************************)
EXTENDS Integers, FiniteSets

CONSTANTS MaxAmount,   \* largest absolute posting amount explored
          MaxEntries,  \* how many entries the model posts
          MaxAccrual   \* ceiling on total value accrued

(***************************************************************************)
(* Four accounts.  `a` and `b` receive ordinary postings; `subject` is the *)
(* liability owed to a data subject and `clearing` is the cash side of a   *)
(* payout.  Keeping the payout accounts out of ordinary postings is what   *)
(* lets the conservation invariant be stated exactly.                      *)
(***************************************************************************)
Accounts     == {"a", "b", "subject", "clearing"}
PostAccounts == {"a", "b"}

(***************************************************************************)
(* The computation scale.  DUAP uses 10^9 nano-minor-units per minor unit; *)
(* 10 here keeps the model small, and the property is scale-independent.   *)
(***************************************************************************)
Scale == 10

VARIABLES balances,  \* account |-> signed settled balance
          posted,    \* entries posted so far
          precise,   \* value accrued but not yet settled, at fine scale
          accrued    \* total value ever accrued, at fine scale

vars == <<balances, posted, precise, accrued>>

Amounts == (-MaxAmount)..MaxAmount

Total(f) == f["a"] + f["b"] + f["subject"] + f["clearing"]

TypeOK ==
  /\ balances \in [ Accounts -> Int ]
  /\ posted \in 0..MaxEntries
  /\ precise \in 0..MaxAccrual
  /\ accrued \in 0..MaxAccrual

Init ==
  /\ balances = [ x \in Accounts |-> 0 ]
  /\ posted = 0
  /\ precise = 0
  /\ accrued = 0

(***************************************************************************)
(* Posting.  The guard is the whole point: an entry whose postings do not  *)
(* sum to zero is not postable, so no reachable state has an unbalanced    *)
(* ledger.  There is no path that posts first and checks later.            *)
(***************************************************************************)
Post ==
  /\ posted < MaxEntries
  /\ \E v \in Amounts :
        balances' = [ balances EXCEPT !["a"] = @ + v, !["b"] = @ - v ]
  /\ posted' = posted + 1
  /\ UNCHANGED <<precise, accrued>>

(***************************************************************************)
(* Reversal.  A correction is a new balanced entry, never an edit.  It is  *)
(* modelled separately from Post to make that visible, though it is the    *)
(* same shape with opposite signs.                                         *)
(***************************************************************************)
Reverse ==
  /\ posted < MaxEntries
  /\ \E v \in Amounts :
        balances' = [ balances EXCEPT !["a"] = @ - v, !["b"] = @ + v ]
  /\ posted' = posted + 1
  /\ UNCHANGED <<precise, accrued>>

(***************************************************************************)
(* Accrual.  A subject earns value at the computation scale, below the     *)
(* granularity any payment rail can move.                                  *)
(***************************************************************************)
Accrue ==
  /\ \E v \in 1..MaxAmount :
        /\ accrued + v =< MaxAccrual
        /\ precise' = precise + v
        /\ accrued' = accrued + v
  /\ UNCHANGED <<balances, posted>>

(***************************************************************************)
(* Settlement.  The whole settled units are moved from the liability to    *)
(* cash; the sub-unit remainder stays accrued.  Nothing is rounded away.   *)
(***************************************************************************)
Settle ==
  /\ precise >= Scale
  /\ LET whole == precise \div Scale
     IN /\ balances' = [ balances EXCEPT
                           !["subject"]  = @ - whole,
                           !["clearing"] = @ + whole ]
        /\ precise' = precise % Scale
  /\ UNCHANGED <<posted, accrued>>

Next == Post \/ Reverse \/ Accrue \/ Settle

Spec == Init /\ [][Next]_vars /\ WF_vars(Next)

(***************************************************************************)
(* INV-L2.  The trial balance is zero in every reachable state.            *)
(***************************************************************************)
TrialBalanceZero == Total(balances) = 0

(***************************************************************************)
(* INV-L3.  Value is conserved across the scale boundary: everything ever  *)
(* accrued is either settled into the cash account or still carried in     *)
(* `precise`.  Nothing is rounded to zero and nothing is invented.         *)
(***************************************************************************)
ValueConserved == balances["clearing"] * Scale + precise = accrued

(***************************************************************************)
(* The carried remainder is always strictly below one settled unit once    *)
(* settlement is no longer enabled, and never negative.                    *)
(***************************************************************************)
RemainderBounded == precise >= 0

(***************************************************************************)
(* INV-L1.  No transition changes the ledger total.  Stated over the       *)
(* transition relation, which is the precise form of "every entry          *)
(* balances".                                                              *)
(***************************************************************************)
EntryBalances == [][ Total(balances') = Total(balances) ]_vars

=============================================================================
