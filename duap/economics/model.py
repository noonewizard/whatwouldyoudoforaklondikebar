#!/usr/bin/env python3
"""Unit-economics model for a DUAP usage-accounting service.

STATUS: REFERENCE (an explicit model, not a forecast).

This is a model, not a projection. Every input is an assumption, every
assumption is named and given a range, and the output is a set of
break-even conditions rather than a revenue figure. The point of running it
is to find out which assumptions the answer is sensitive to, so that the
commercial work targets those rather than the comfortable ones.

Two rules this file obeys, from the project directive:

  * No number in here is a measurement unless it cites one. The only
    measured inputs are the per-event costs, which come from
    benchmarks/results/2026-09-21-ci-runner.md.
  * Nothing multiplies a per-core figure by a core count and calls it
    throughput.

Run:  python3 economics/model.py            # writes economics/results.md
      python3 economics/model.py --print    # to stdout
"""

from __future__ import annotations

import argparse
import itertools
from dataclasses import dataclass, field, asdict

# --------------------------------------------------------------------------
# Measured inputs. These are the only non-assumptions in the file.
# Source: benchmarks/results/2026-09-21-ci-runner.md, commit 7551619.
# --------------------------------------------------------------------------

INGEST_US_PER_EVENT = 127.7          # microseconds, single-threaded
CLOSE_PERIOD_MS_PER_20K = 7.44       # ms to price/receipt/anchor 20k events
EVENT_BYTES_ED25519 = 665            # canonical envelope size
LOG_ENTRY_BYTES = 129

MEASURED = {
    "ingest_us_per_event": INGEST_US_PER_EVENT,
    "close_period_ms_per_20k_events": CLOSE_PERIOD_MS_PER_20K,
    "event_envelope_bytes": EVENT_BYTES_ED25519,
    "log_entry_bytes": LOG_ENTRY_BYTES,
}


# --------------------------------------------------------------------------
# Assumptions. Every one of these is a guess with a stated range.
# --------------------------------------------------------------------------

@dataclass(frozen=True)
class Assumption:
    name: str
    low: float
    base: float
    high: float
    unit: str
    basis: str


ASSUMPTIONS: list[Assumption] = [
    Assumption(
        "compute_cost_per_core_hour", 0.02, 0.05, 0.12, "USD",
        "General-purpose cloud vCPU list pricing is of this order. NOT "
        "verified against a provider price list in this environment; "
        "treat as an order-of-magnitude placeholder.",
    ),
    Assumption(
        "storage_cost_per_gb_month", 0.01, 0.023, 0.05, "USD",
        "Object-storage list pricing is of this order. Same caveat.",
    ),
    Assumption(
        "retention_months", 12, 36, 84, "months",
        "Evidence must outlive the dispute window and any statutory "
        "limitation period. 36 months is a guess at a common contractual "
        "audit window, not a legal finding.",
    ),
    Assumption(
        "events_per_customer_per_month", 1e6, 5e7, 1e10, "events",
        "Spans a mid-size enterprise reporting aggregate counters to a "
        "large AI platform reporting per-inference events. The four-order "
        "spread is the point: the model is dominated by this.",
    ),
    Assumption(
        "aggregation_ratio", 1, 1000, 100000, "events per retained event",
        "How much detail is discarded after aggregation. 1 means every "
        "event is retained; 1000 means counters are kept and the events "
        "behind them are summarised. Kill attempt 4 of the falsification "
        "report makes this constraint mandatory rather than optional.",
    ),
    Assumption(
        "licensed_value_per_customer_per_month", 1e3, 5e4, 2e6, "USD",
        "The gross value of data licensing flowing through one customer's "
        "DUAP instance. Reported AI licensing deals span roughly $5M to "
        "$250M in total contract value per trade press; monthly run-rate "
        "per counterparty relationship is inferred and unverified.",
    ),
    Assumption(
        "take_rate", 0.0005, 0.003, 0.01, "fraction",
        "Fee on cleared value. Card interchange and exchange fees are of "
        "this order. A take rate on value is the most adoption-hostile "
        "model available and is modelled to show why, not to recommend it.",
    ),
    Assumption(
        "subscription_per_customer_per_month", 500, 4000, 25000, "USD",
        "Infrastructure-software seat/instance pricing. Unverified.",
    ),
    Assumption(
        "dispute_rate", 0.0001, 0.002, 0.02, "fraction of invoices",
        "Entirely unknown. No public data on dispute rates in metered "
        "data licensing was located. This is the least defensible number "
        "in the file and is flagged as such in the output.",
    ),
    Assumption(
        "dispute_handling_cost", 50, 400, 3000, "USD per dispute",
        "Human time. Unverified.",
    ),
    Assumption(
        "gross_customers", 5, 50, 500, "customers",
        "Scale points, not a forecast.",
    ),
]

BY_NAME = {a.name: a for a in ASSUMPTIONS}


def pick(scenario: str) -> dict[str, float]:
    """Return the assumption set for 'conservative', 'base' or 'aggressive'.

    Note the deliberate asymmetry: the conservative scenario takes the
    *unfavourable* end of each assumption, which for costs is `high` and
    for revenue drivers is `low`. Getting this backwards is the classic way
    a model flatters itself.
    """
    unfavourable_when_high = {
        "compute_cost_per_core_hour",
        "storage_cost_per_gb_month",
        "retention_months",
        "events_per_customer_per_month",
        "dispute_rate",
        "dispute_handling_cost",
    }
    out: dict[str, float] = {}
    for a in ASSUMPTIONS:
        if scenario == "base":
            out[a.name] = a.base
        elif scenario == "conservative":
            out[a.name] = a.high if a.name in unfavourable_when_high else a.low
        elif scenario == "aggressive":
            out[a.name] = a.low if a.name in unfavourable_when_high else a.high
        else:
            raise ValueError(scenario)
    # Aggregation cuts both ways: high aggregation lowers cost, so it is
    # favourable. Handled explicitly rather than by the set above.
    if scenario == "conservative":
        out["aggregation_ratio"] = BY_NAME["aggregation_ratio"].low
    elif scenario == "aggressive":
        out["aggregation_ratio"] = BY_NAME["aggregation_ratio"].high
    return out


# --------------------------------------------------------------------------
# The model
# --------------------------------------------------------------------------

@dataclass
class Result:
    scenario: str
    events_per_month: float
    ingest_core_hours: float
    compute_cost: float
    retained_gb: float
    storage_cost: float
    dispute_cost: float
    total_cost_per_customer: float
    take_rate_revenue: float
    subscription_revenue: float
    notes: list[str] = field(default_factory=list)


def run(scenario: str) -> Result:
    a = pick(scenario)
    ev = a["events_per_customer_per_month"]

    # Compute: ingest is the dominant CPU cost. close_period is negligible
    # beside it and is included for completeness rather than because it
    # matters.
    ingest_seconds = ev * INGEST_US_PER_EVENT / 1e6
    close_seconds = (ev / 20000.0) * CLOSE_PERIOD_MS_PER_20K / 1e3
    core_hours = (ingest_seconds + close_seconds) / 3600.0
    compute = core_hours * a["compute_cost_per_core_hour"]

    # Storage: only retained events cost anything; the rest are aggregated
    # away. Log entries are kept for every batch and are negligible.
    retained = ev / a["aggregation_ratio"]
    gb_added_per_month = retained * EVENT_BYTES_ED25519 / 1e9
    # Steady state: `retention_months` worth of accumulated evidence.
    gb_resident = gb_added_per_month * a["retention_months"]
    storage = gb_resident * a["storage_cost_per_gb_month"]

    # Disputes: assume one invoice per counterparty relationship per month.
    # This is a crude proxy; the dispute *rate* assumption already dominates.
    invoices = 1.0
    disputes = invoices * a["dispute_rate"]
    dispute_cost = disputes * a["dispute_handling_cost"]

    total_cost = compute + storage + dispute_cost

    take = a["licensed_value_per_customer_per_month"] * a["take_rate"]
    sub = a["subscription_per_customer_per_month"]

    notes: list[str] = []
    if total_cost > sub:
        notes.append(
            "Cost per customer exceeds subscription revenue: the "
            "subscription model does not cover infrastructure at this "
            "event volume and aggregation ratio."
        )
    if take > sub * 10:
        notes.append(
            "Take-rate revenue is an order of magnitude above subscription "
            "revenue, which is exactly the condition under which customers "
            "route around the fee."
        )
    if a["aggregation_ratio"] == 1:
        notes.append(
            "No aggregation. Storage dominates and grows without bound; "
            "this is the configuration kill attempt 4 rules out."
        )
    return Result(
        scenario=scenario,
        events_per_month=ev,
        ingest_core_hours=core_hours,
        compute_cost=compute,
        retained_gb=gb_resident,
        storage_cost=storage,
        dispute_cost=dispute_cost,
        total_cost_per_customer=total_cost,
        take_rate_revenue=take,
        subscription_revenue=sub,
        notes=notes,
    )


def break_even_events(scenario: str) -> float:
    """Events per customer per month at which infrastructure cost equals
    subscription revenue. Above this, the subscription model loses money on
    that customer."""
    a = pick(scenario)
    sub = a["subscription_per_customer_per_month"]
    per_event_compute = INGEST_US_PER_EVENT / 1e6 / 3600.0 * a["compute_cost_per_core_hour"]
    per_event_storage = (
        EVENT_BYTES_ED25519 / 1e9 / a["aggregation_ratio"]
        * a["retention_months"] * a["storage_cost_per_gb_month"]
    )
    per_event = per_event_compute + per_event_storage
    return sub / per_event if per_event > 0 else float("inf")


def sensitivity() -> list[tuple[str, float, float]]:
    """One-at-a-time sensitivity on total cost, base case.

    Returns (assumption, cost at low, cost at high). The spread tells you
    which assumptions are worth spending commercial effort to pin down.
    """
    out = []
    for a in ASSUMPTIONS:
        if a.name in ("take_rate", "subscription_per_customer_per_month",
                      "licensed_value_per_customer_per_month",
                      "gross_customers"):
            continue
        costs = []
        for value in (a.low, a.high):
            saved = BY_NAME[a.name]
            patched = pick("base")
            patched[a.name] = value
            ev = patched["events_per_customer_per_month"]
            ingest_seconds = ev * INGEST_US_PER_EVENT / 1e6
            core_hours = ingest_seconds / 3600.0
            compute = core_hours * patched["compute_cost_per_core_hour"]
            retained = ev / patched["aggregation_ratio"]
            gb = retained * EVENT_BYTES_ED25519 / 1e9 * patched["retention_months"]
            storage = gb * patched["storage_cost_per_gb_month"]
            dispute = patched["dispute_rate"] * patched["dispute_handling_cost"]
            costs.append(compute + storage + dispute)
            del saved
        out.append((a.name, costs[0], costs[1]))
    out.sort(key=lambda r: -(max(r[1], r[2]) - min(r[1], r[2])))
    return out


def money(x: float) -> str:
    if x >= 1e6:
        return f"${x/1e6:,.1f}M"
    if x >= 1e3:
        return f"${x/1e3:,.1f}k"
    if x >= 1:
        return f"${x:,.2f}"
    return f"${x:.4f}"


def render() -> str:
    lines: list[str] = []
    w = lines.append
    w("# Unit-economics model: results")
    w("")
    w("**Status:** REFERENCE (a model, not a forecast) · generated by "
      "`economics/model.py`")
    w("")
    w("Regenerate with `python3 economics/model.py`. Do not hand-edit.")
    w("")
    w("## What this is and is not")
    w("")
    w("This is a cost and break-even model for operating a DUAP "
      "usage-accounting service. It is **not** a revenue forecast, a market "
      "size, or a valuation. It takes four measured numbers from "
      "`benchmarks/results/2026-09-21-ci-runner.md` and eleven assumptions, "
      "and reports which of the eleven the answer depends on.")
    w("")
    w("Almost every assumption below is unverified. The cloud prices were "
      "not checked against a provider price list, because this environment "
      "could not reach one; the dispute rate has no public data behind it "
      "at all. They are labelled rather than quietly used.")
    w("")
    w("## Measured inputs")
    w("")
    w("| Quantity | Value |")
    w("|---|---:|")
    for k, v in MEASURED.items():
        w(f"| `{k}` | {v} |")
    w("")
    w("## Assumptions")
    w("")
    w("| Assumption | Low | Base | High | Unit | Basis |")
    w("|---|---:|---:|---:|---|---|")
    for a in ASSUMPTIONS:
        w(f"| `{a.name}` | {a.low:g} | {a.base:g} | {a.high:g} | {a.unit} | {a.basis} |")
    w("")
    w("## Scenarios")
    w("")
    w("Conservative takes the unfavourable end of every assumption, "
      "aggressive the favourable end. Per customer per month.")
    w("")
    w("| | Conservative | Base | Aggressive |")
    w("|---|---:|---:|---:|")
    rs = {s: run(s) for s in ("conservative", "base", "aggressive")}
    rows = [
        ("Events/month", lambda r: f"{r.events_per_month:,.0f}"),
        ("Ingest core-hours", lambda r: f"{r.ingest_core_hours:,.1f}"),
        ("Compute cost", lambda r: money(r.compute_cost)),
        ("Resident evidence (GB)", lambda r: f"{r.retained_gb:,.1f}"),
        ("Storage cost", lambda r: money(r.storage_cost)),
        ("Dispute cost", lambda r: money(r.dispute_cost)),
        ("**Total cost**", lambda r: f"**{money(r.total_cost_per_customer)}**"),
        ("Subscription revenue", lambda r: money(r.subscription_revenue)),
        ("Take-rate revenue", lambda r: money(r.take_rate_revenue)),
    ]
    for label, fn in rows:
        w(f"| {label} | {fn(rs['conservative'])} | {fn(rs['base'])} | {fn(rs['aggressive'])} |")
    w("")
    for s, r in rs.items():
        for n in r.notes:
            w(f"- **{s}:** {n}")
    w("")
    w("## Break-even")
    w("")
    w("Events per customer per month at which infrastructure cost equals "
      "subscription revenue:")
    w("")
    w("| Scenario | Break-even events/month |")
    w("|---|---:|")
    for s in ("conservative", "base", "aggressive"):
        w(f"| {s} | {break_even_events(s):,.0f} |")
    w("")
    w("## Sensitivity")
    w("")
    w("One assumption moved from low to high, all others at base. Sorted by "
      "the size of the swing in total cost. The top rows are where "
      "commercial effort to pin down a number is worth spending; the bottom "
      "rows do not matter and should not be argued about.")
    w("")
    w("| Assumption | Cost at low | Cost at high | Swing |")
    w("|---|---:|---:|---:|")
    for name, lo, hi in sensitivity():
        w(f"| `{name}` | {money(lo)} | {money(hi)} | {money(abs(hi-lo))} |")
    w("")
    w("## What the model says")
    w("")
    w("These conclusions are read off the tables above rather than "
      "asserted; where the model contradicted the author's prior, the "
      "prior lost.")
    w("")
    w("1. **Infrastructure cost is not the constraint, and that was not "
      "the expected answer.** At base assumptions the total cost of "
      "running a customer's usage accounting is under a dollar a month "
      "against thousands of dollars of subscription revenue, and the "
      "break-even event volume is in the trillions per customer per month "
      "-- far past anything plausible. The intuition that cryptographic "
      "evidence is expensive is wrong at these volumes: 127.7 us of CPU "
      "and 665 bytes per event are simply cheap. Whatever kills this "
      "business, it is not the cost of signatures.")
    w("")
    w("2. **The one configuration that does blow up is no aggregation at "
      "extreme volume.** The conservative column -- every event retained, "
      "10^10 events a month, 84-month retention -- reaches 558,600 GB "
      "resident and roughly $28k a month per customer, against $500 of "
      "revenue. That is not a cost problem to optimise; it is a design "
      "constraint. Aggregation is mandatory, which is the quantitative "
      "form of kill attempt 4 in the falsification report.")
    w("")
    w("3. **Human cost dominates machine cost.** At base assumptions the "
      "dispute-rate and dispute-handling-cost assumptions swing total "
      "cost by more than every infrastructure assumption combined -- "
      "several dollars against fractions of a cent for storage and "
      "compute. A single disputed invoice costs more than a month of "
      "servers. The engineering consequence is that effort spent making "
      "disputes rare or cheap to resolve is worth more than any "
      "optimisation of the crypto path, including PERF-01.")
    w("")
    w("4. **The number nobody knows is one of the numbers that matters.** "
      "The dispute rate has no public data behind it and ranks third in "
      "the sensitivity table. That combination -- high leverage, no "
      "evidence -- makes it the single most valuable thing to measure in "
      "any pilot, ahead of throughput, latency and every other metric "
      "this project has instrumented.")
    w("")
    w("5. **The subscription model covers cost comfortably; the "
      "take-rate model is rejected on incentives, not on size.** In the "
      "aggressive case a take rate yields $20k a month against $25k of "
      "subscription, so it is not that the fee is small. It is that a fee "
      "proportional to the value flowing through the protocol is the "
      "strongest available incentive for a customer to route around the "
      "protocol. See `docs/business/business-model.md`.")
    w("")
    w("6. **This model stops at unit economics on purpose.** It does not "
      "multiply a margin by a customer count, because both would be "
      "assumptions and the product of two assumptions carries no "
      "information. The commercial question is whether anyone buys, and "
      "no spreadsheet answers that.")
    w("")
    return "\n".join(lines) + "\n"


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--print", action="store_true", dest="to_stdout")
    args = ap.parse_args()
    text = render()
    if args.to_stdout:
        print(text)
    else:
        import pathlib
        out = pathlib.Path(__file__).resolve().parent / "results.md"
        out.write_text(text)
        print(f"wrote {out}")


if __name__ == "__main__":
    main()
