# Competitive landscape

**Status:** REFERENCE · 2026-09-21
**Owner:** `competitive-red-team`, reviewed by `chief-architect`

**Retrieval limits.** On 2026-09-21 this environment's network policy
blocked outbound HTTPS to every host except `github.com`. Every company and
product fact below arrived through a web-search tool's generated summary of
pages that could not be opened, and is marked **[ss]**. A URL beside such a
claim is where the claim came from, not a document this author read. No
company, product or figure below is invented; several are reported at one
remove and should be verified against primary sources before any of this is
acted on.

The instruction this document follows is "do not invent competitors, use
current evidence". Where evidence was unavailable the row says so rather
than being filled in plausibly.

---

## 0. The finding that matters most

The single most important thing in this document is that **the AI web-content
vertical — the beachhead DUAP would most naturally have chosen — already has
a rights-expression standard and a deployed metering incumbent.**

- **Really Simple Licensing (RSL)** is an open, XML-based standard for
  machine-readable licensing and compensation terms, launched 10 September
  2025, with RSL 1.0 published 10 December 2025, managed by the non-profit
  RSL Collective. It supports free, attribution, subscription,
  **pay-per-crawl** and **pay-per-inference** models, and is discoverable
  through `robots.txt`, HTTP headers, RSS and HTML link elements. **[ss]**
  ([RSL 1.0](https://rslstandard.org/rsl);
  [announcement](https://rslstandard.org/press/rsl-1-specification-2025);
  [The Register](https://www.theregister.com/2025/12/10/really_simple_licensing_spec_takes/))

- **Cloudflare** launched pay-per-crawl in July 2025, using HTTP 402
  Payment Required as the signal, with a minimum price of $0.001 per
  successful retrieval and publisher-set rates; publishers are reported to
  send more than one billion 402 responses to AI crawlers per day across
  Cloudflare's network, with early adopters including Condé Nast, Time, the
  Associated Press, BuzzFeed, Reddit, Pinterest and Stack Overflow. **[ss]**
  ([TechCrunch](https://techcrunch.com/2025/07/01/cloudflare-launches-a-marketplace-that-lets-websites-charge-ai-bots-for-scraping/);
  [Cloudflare blog](https://blog.cloudflare.com/introducing-ai-crawl-control/);
  [Stack Overflow](https://stackoverflow.blog/2026/02/19/stack-overflow-cloudflare-pay-per-crawl/))

- An intermediary layer of **more than a dozen companies** has formed since
  2024, including TollBit, ProRata, ScalePost, Sphere AI, Created by Humans
  and Miso.ai, alongside Cloudflare and Microsoft. TollBit prices bot access
  per 1,000 pages scraped; ProRata shares advertising revenue from AI answers
  that cite publisher content and was reported to have over 500 publishers
  signed up. **[ss]**
  ([Brookings](https://www.brookings.edu/articles/same-gatekeepers-new-tollbooths-in-the-ai-content-licensing-market/);
  [Digiday](https://digiday.com/media/the-case-for-and-against-publisher-content-marketplaces/);
  [Nieman Lab](https://www.niemanlab.org/2026/05/the-emerging-ai-content-licensing-market-puts-news-publishers-in-a-double-bind-a-new-report-warns/))

This is not a gap. It is a crowded, funded, deployed market with an open
standard and an infrastructure incumbent operating at web scale, and it
closed while this protocol was being designed.

**What it does to the plan** is set out in §5 and in kill attempt 11 of
`docs/reviews/falsification.md`. In short: the AI web-content vertical is
removed as a beachhead, and what remains of DUAP's claim in that vertical
is one specific thing — **pay-per-inference is a term RSL can express and
nobody can currently measure or prove**, because inference happens inside
the buyer's infrastructure where no crawler gate can see it.

---

## 1. Rights expression and licensing standards

| Project | What it does | Economic model | Evidence model | Relationship to DUAP |
|---|---|---|---|---|
| **RSL 1.0** [ss] | XML vocabulary declaring licensing and compensation terms for AI access; discoverable via robots.txt, HTTP headers, RSS, HTML | Declares free / attribution / subscription / pay-per-crawl / pay-per-inference terms | **None.** Terms are declared *prior to* use; the standard does not define usage reporting, receipts or audit | **Direct overlap on terms, no overlap on evidence.** A DUAP grant should be derivable from an RSL licence. RSL is more deployable than ODRL for web content and DUAP should bind it, not compete |
| **W3C ODRL 2.2** [ss] | General policy expression language, W3C Recommendation 15 Feb 2018 | Duties can reference payment | None | Bind; see `docs/standards/prior-art.md` |
| **IDS Dataspace Protocol 2025-1** | Catalogue, contract negotiation, transfer initiation between dataspace participants; TCK; heading to ISO | Contract agreements | No usage receipt; specification states it does not address metering, billing, accounting, clearing or settlement (**retrieved**, 2026-09-21) | Complementary. Owns the pre-usage half |
| **Gaia-X Trust Framework** [ss] | Federated trust and compliance framework | — | Credential-based | Cautionary precedent on adoption |

**The pattern:** every rights standard in this table stops at declaring
terms. None closes the loop by reporting what was then done. RSL is the
sharpest example, because it declares a pay-per-inference term that has no
measurement mechanism anywhere in its stack.

## 2. Metering and enforcement points

| Product | What it meters | Who holds the meter | Verifiable by the counterparty? | Relationship to DUAP |
|---|---|---|---|---|
| **Cloudflare AI Crawl Control / pay-per-crawl** [ss] | HTTP requests by identified AI crawlers, at the CDN edge | Cloudflare | **No.** The publisher and the AI company both trust Cloudflare's count | The most serious competitive fact in this document. Also the clearest demonstration that metered data licensing is real |
| **TollBit** [ss] | Pages scraped, priced per 1,000 | TollBit | No | Same shape, smaller footprint |
| **ProRata** [ss] | AI answers citing publisher content; revenue share | ProRata | No | Attribution-based revenue share, which this project's own research says cannot be grounded in influence — ProRata's is a contractual apportionment, not a measurement |
| **OpenTelemetry** | Anything, inside one organisation | The instrumented system | No; unsigned self-report | Bind as an instrumentation substrate |
| **Cloud billing / FinOps FOCUS** [ss] | Provider resource consumption | The provider | No; trust the seller's billing | Design precedent |

**The structural observation, and DUAP's actual differentiator:** every
deployed meter in this table is a **trusted third party or a
self-report**. The publisher trusts Cloudflare; the AI company trusts
Cloudflare; neither can check. That model works while one intermediary sits
in the path and both sides accept it. It does not extend to usage that
happens inside the buyer — inference, fine-tuning, derivation, internal
redistribution — because there is no path for an intermediary to sit in.

DUAP's claim reduces to: *evidence that does not require trusting the
party that produced it, for usage no intermediary can observe.* That is
narrower than "the interoperability layer for the data economy" and it is
defensible.

**The honest counter:** trusted third parties are cheap, work today, and
are what the market is buying. "Cryptographically verifiable" has lost to
"good enough and already deployed" many times.

## 3. Data marketplaces

| Platform | Model | Reported scale | Overlap with DUAP |
|---|---|---|---|
| **Snowflake Marketplace** [ss] | In-ecosystem data sharing and monetisation | 820+ providers; 3,400+ listings reported as of May 2026 ([Bright Data](https://brightdata.com/blog/web-data/best-data-marketplaces)) | None on accounting. A venue, not a protocol; usage accounting is internal to Snowflake |
| **Databricks Marketplace / Delta Sharing** [ss] | Open sharing protocol plus marketplace | — | Delta Sharing is the closest thing to an open *access* protocol; it shares data, it does not account for use |
| **AWS Data Exchange** [ss] | Subscription data products inside AWS | — | Same |
| **Datarade** [ss] | Neutral B2B discovery across vendors | 2,000+ providers reported | Discovery, not accounting |
| **Dawex** [ss] | Neutral data exchange platform | — | Same |

**Why marketplaces are not the competition:** a marketplace is a venue that
solves discovery and contracting. None of them produces an evidence object
that survives leaving the venue — which is the whole point of an
interoperability layer, and also why marketplaces do not experience the
absence of one as a problem. They are potential *adopters*, not rivals, and
they have no incentive to adopt while their customers are inside their
walls.

## 4. Personal data marketplaces and data unions

| Project | Status | What it tells us |
|---|---|---|
| **Datacoup** [ss] | Founded 2012, reported shut down November 2019 ([Wikipedia](https://en.wikipedia.org/wiki/Data_marketplace)) | The canonical failure. Reported causes are the ones the arithmetic predicts: too little money per user, too few users, nothing worth selling |
| **Streamr Data Unions** [ss] | Reported operating | Token-mediated bundling of user data. Evidence of the collective model's durability is UNKNOWN from these sources |
| **Swash** [ss] | Reported operating | Browser-plugin data union |
| **RSL Collective** [ss] | Founded 2025 | A *collective rights organisation* for web publishers — the collective-licensing model succeeding where individual licensing failed, which is the same conclusion §2 of the falsification report reaches |

**What this category establishes:** individual data monetisation has a
long record of failure and collective licensing has a live, current,
credible instance. That is a strong argument for the cooperative model over
the wallet model, and it is why `docs/market/consumer.md` contains no
marketplace.

## 5. What this does to DUAP's positioning

**Removed:** AI web-content licensing as the beachhead. RSL owns terms
declaration, Cloudflare owns the crawl gate, and a dozen intermediaries own
the commercial layer. Entering there means competing with a deployed
incumbent at web scale on its strongest ground, with no differentiated
capability at the crawl boundary — Cloudflare's meter is good enough for
crawl because Cloudflare is in the path.

**Sharpened:** the residual claim, in three parts, each of which the table
in §2 shows nobody serving.

1. **Usage no intermediary can observe.** Inference, fine-tuning,
   derivation and internal redistribution happen inside the buyer. RSL
   declares pay-per-inference terms; no deployed system measures inference.
   A signed self-report with non-repudiation, sequence-gap detection and
   cross-reporter double-count detection is strictly better than nothing,
   which is what exists today — and is honestly weaker than a gate.
2. **Evidence that does not require trusting the meter's owner.** Every
   deployed meter is a trusted third party. That is a governance
   concentration as well as a technical one, and Brookings' framing of the
   market as "same gatekeepers, new tollbooths" **[ss]** is the political
   version of the same observation.
3. **Accounting across many counterparties.** One publisher against one
   crawler needs no clearing. A thousand rights-holders against a dozen
   buyers under metered terms is a netting problem, and none of the
   platforms above nets across platforms.

**Unchanged and still unresolved:** whether any buyer wants (1). A buyer
asked to meter and sign its own internal use, for a counterparty's benefit,
has an obvious reason to decline. Nothing in this landscape resolves that,
and it remains kill attempt 7.

## 6. What would change this assessment

- RSL adding a usage-reporting or receipt mechanism. This is the most
  likely way the residual claim disappears in the AI vertical, and the RSL
  Collective has both the motive and the standing to do it.
- Cloudflare extending beyond the crawl boundary into inference-time
  accounting, or publishing its metering as a verifiable rather than
  trusted artefact.
- The Dataspace Protocol adding usage accounting, which would close the
  enterprise side the same way.
- Evidence that pay-per-inference terms are not being agreed in practice,
  which would remove the one measurable gap this document identifies.

Each of these is a reason to narrow or stop, and each should be checked
before further engineering. The correct next action after this document is
not more code.
