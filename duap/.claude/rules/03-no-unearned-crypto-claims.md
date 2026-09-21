# No unearned cryptographic or security claims

A security property is claimed only with a written argument that names:

1. **The property**, stated precisely enough to be false.
2. **The adversary**, including what it can observe, corrupt and compute.
3. **The assumption** the property rests on (a hardness assumption, a trust
   assumption, an operational assumption).
4. **What the property does not cover.**
5. **The test or proof** that exercises it.

## Requirements

- "Secure", "tamper-proof", "unforgeable", "private", "anonymous",
  "immutable" and "verifiable" are claims. Each needs the five items above or
  must be replaced with a precise statement.
- A property that depends on an operational behaviour (key hygiene, gossip
  between auditors, timely revocation publication) is stated as conditional,
  with the condition named.
- A cryptographic primitive is used only if it is standardised and published.
  The implementation used is named, along with whether it has been
  independently audited -- and if it has not, that is stated.
- Detection guarantees are never described as prevention guarantees. A
  transparency log detects equivocation; it does not prevent it.

## Where the arguments live

`SECURITY.md` holds the arguments; `THREAT_MODEL.md` holds the adversaries;
the test names are cited from both.
