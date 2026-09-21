#!/usr/bin/env python3
"""Author the DUAP v1 ontology as JSON.

The JSON file is the normative artefact; this script is how it is edited.
Running it must be idempotent -- CI checks that the committed JSON matches.
"""
import json, pathlib, sys

# ---------------------------------------------------------------------------
# Sensitivity tiers
# ---------------------------------------------------------------------------
# Tiers order data by the harm a disclosure would do to the data subject. They
# are inputs to pricing and to policy, never a legal classification: whether a
# class is a "special category" under a given regime is recorded separately and
# per regime, because the regimes disagree.
TIERS = [
    {"code": "t0", "name": "non-personal", "rank": 0,
     "description": "No link to an identified or identifiable natural person is expected."},
    {"code": "t1", "name": "low", "rank": 1,
     "description": "Personal but widely observable; disclosure is an annoyance."},
    {"code": "t2", "name": "moderate", "rank": 2,
     "description": "Personal and not widely observable; disclosure enables profiling."},
    {"code": "t3", "name": "high", "rank": 3,
     "description": "Disclosure enables targeting, discrimination or financial loss."},
    {"code": "t4", "name": "severe", "rank": 4,
     "description": "Disclosure risks physical safety, liberty, or irreversible harm."},
]

# ---------------------------------------------------------------------------
# Data classes
# ---------------------------------------------------------------------------
# reid = baseline re-identification risk of a single record of this class in
# isolation, on a 0..100 scale. Documented as a prior, not a measurement: the
# valuation engine treats it as an adjustable parameter and the calibration
# procedure is in VALUATION.md.
def dc(code, name, tier, reid, special=(), notes=""):
    return {"code": code, "name": name, "sensitivity": tier, "reid_risk": reid,
            "special_category": list(special), "notes": notes}

DATA_CLASSES = [
    # identity
    dc("identity.direct", "Direct identifier", "t3", 95, (),
       "Legal name, national identifier, passport number."),
    dc("identity.pseudonymous", "Pseudonymous identifier", "t2", 60, (),
       "Stable identifier with no direct link to a legal identity."),
    dc("identity.government", "Government-issued identifier", "t4", 98, ("us.ssn",)),
    dc("contact.email", "Email address", "t2", 85),
    dc("contact.phone", "Telephone number", "t2", 88),
    dc("contact.postal", "Postal address", "t3", 80),
    # location
    dc("location.precise", "Precise geolocation", "t4", 92, ("ccpa.precise_geolocation",),
       "Finer than 1850 m; CPRA treats this as sensitive personal information."),
    dc("location.coarse", "Coarse geolocation", "t2", 45),
    dc("location.inferred_home", "Inferred home location", "t4", 90),
    dc("location.inferred_work", "Inferred workplace", "t3", 78),
    dc("location.trajectory", "Movement trajectory", "t4", 96,
       ("ccpa.precise_geolocation",),
       "Four spatio-temporal points identify 95% of individuals (de Montjoye et al. 2013)."),
    # device
    dc("device.identifier", "Device or advertising identifier", "t2", 70),
    dc("device.fingerprint", "Passive device fingerprint", "t3", 82, (),
       "Constructed without storage on the device; typically outside cookie consent flows."),
    dc("device.configuration", "Device configuration", "t1", 35),
    dc("device.network", "Network address and topology", "t2", 66),
    # behaviour
    dc("behavior.web_browsing", "Web browsing activity", "t2", 72),
    dc("behavior.app_usage", "Application usage", "t2", 64),
    dc("behavior.search", "Search queries", "t3", 80),
    dc("behavior.content_consumption", "Content consumption", "t2", 58),
    dc("behavior.social_graph", "Social connections", "t3", 86),
    dc("behavior.keystroke_dynamics", "Keystroke or interaction dynamics", "t3", 84,
       (), "Behavioural biometric in several regimes."),
    dc("behavior.attention", "Gaze, dwell and attention signals", "t3", 70),
    # transactions and finance
    dc("transaction.purchase", "Purchase record", "t2", 68),
    dc("transaction.payment_instrument", "Payment instrument data", "t4", 90,
       ("pci.chd",)),
    dc("financial.account", "Financial account data", "t4", 88, ("us.glba.npi",)),
    dc("financial.creditworthiness", "Credit assessment input", "t4", 85,
       ("us.fcra",)),
    dc("financial.income", "Income and employment", "t3", 75),
    # content
    dc("content.user_generated", "User-generated content", "t3", 76),
    dc("content.communications", "Interpersonal communications", "t4", 90,
       ("us.ecpa",)),
    dc("content.media", "Photographs, audio or video", "t3", 82),
    # biometric and health
    dc("biometric.template", "Biometric template", "t4", 99,
       ("gdpr.art9", "us.il.bipa", "ccpa.biometric")),
    dc("biometric.derived", "Biometric-derived inference", "t4", 88,
       ("gdpr.art9", "ccpa.biometric")),
    dc("health.clinical", "Clinical health record", "t4", 94,
       ("gdpr.art9", "us.hipaa.phi")),
    dc("health.wearable", "Wearable physiological telemetry", "t3", 80,
       ("gdpr.art9",)),
    dc("health.inferred", "Inferred health status", "t4", 86, ("gdpr.art9",),
       "Inference, not observation; often escapes sectoral regimes while carrying the same harm."),
    # protected characteristics
    dc("protected.race_ethnicity", "Racial or ethnic origin", "t4", 70,
       ("gdpr.art9", "ccpa.sensitive")),
    dc("protected.religion", "Religious or philosophical belief", "t4", 70,
       ("gdpr.art9", "ccpa.sensitive")),
    dc("protected.political", "Political opinion or union membership", "t4", 72,
       ("gdpr.art9", "ccpa.sensitive")),
    dc("protected.sexual_orientation", "Sex life or sexual orientation", "t4", 74,
       ("gdpr.art9", "ccpa.sensitive")),
    dc("protected.genetic", "Genetic data", "t4", 99,
       ("gdpr.art9", "us.gina")),
    dc("protected.immigration", "Immigration or citizenship status", "t4", 76,
       ("gdpr.art9",)),
    # minors and education
    dc("minor.any", "Data concerning a minor", "t4", 80,
       ("us.coppa", "gdpr.art8")),
    dc("education.record", "Education record", "t3", 78, ("us.ferpa",)),
    # workplace and enterprise
    dc("employment.record", "Employment record", "t3", 80),
    dc("telemetry.enterprise", "Enterprise system telemetry", "t1", 25, (),
       "May carry personal data as a by-product; classification is per-field."),
    dc("telemetry.security", "Security and access telemetry", "t2", 55),
    # machines, vehicles, industry, science
    dc("telemetry.vehicle", "Vehicle telemetry", "t3", 80, (),
       "EU Data Act art. 4-5 give the user access and sharing rights over this."),
    dc("telemetry.iot_home", "Home IoT telemetry", "t3", 74),
    dc("telemetry.iot_industrial", "Industrial IoT telemetry", "t0", 5),
    dc("sensor.environmental", "Environmental sensor data", "t0", 3),
    dc("sensor.imaging_public", "Imaging of public space", "t3", 70),
    dc("industrial.process", "Industrial process data", "t0", 4),
    dc("scientific.measurement", "Scientific measurement", "t0", 4),
    dc("scientific.human_subject", "Human-subject research data", "t4", 88,
       ("us.common_rule", "gdpr.art9")),
    # derived and synthetic
    dc("derived.profile", "Derived behavioural profile", "t3", 84),
    dc("derived.segment", "Audience or risk segment membership", "t2", 50),
    dc("derived.embedding", "Learned representation (embedding)", "t3", 65, (),
       "Embeddings can be inverted to a degree that depends on the model; see AI_ATTRIBUTION.md."),
    dc("derived.aggregate", "Aggregate statistic", "t1", 12),
    dc("derived.dp_aggregate", "Differentially private aggregate", "t0", 2, (),
       "Risk bound holds only for the declared epsilon and only if accounting is correct."),
    dc("derived.synthetic", "Synthetic record", "t1", 20, (),
       "Not automatically non-personal: memorisation can reproduce training records."),
    dc("derived.model_parameters", "Model parameters", "t2", 40),
]

# ---------------------------------------------------------------------------
# Collection methods
# ---------------------------------------------------------------------------
def cm(code, name, observable, notes=""):
    """observable: whether a data subject can, in principle, detect it locally."""
    return {"code": code, "name": name, "subject_observable": observable, "notes": notes}

COLLECTION_METHODS = [
    cm("explicit.form", "Explicit submission by the subject", True),
    cm("explicit.upload", "File or media upload", True),
    cm("passive.cookie", "Cookie or equivalent local storage", True),
    cm("passive.pixel", "Tracking pixel or beacon", True),
    cm("passive.sdk", "Embedded SDK telemetry", False),
    cm("passive.server_log", "Server-side log", False),
    cm("passive.fingerprint", "Device fingerprinting", False,
       "Stateless; not visible in storage inspection."),
    cm("passive.network", "Network-level observation", False),
    cm("device.sensor", "Device sensor read", True),
    cm("device.os_api", "Operating-system API", True),
    cm("observation.camera", "Camera or imaging", False),
    cm("observation.audio", "Microphone or acoustic", False),
    cm("machine.vehicle_bus", "Vehicle data bus", False),
    cm("machine.industrial_bus", "Industrial fieldbus or historian", False),
    cm("acquisition.third_party", "Acquired from a third party", False),
    cm("acquisition.broker", "Acquired from a data broker", False),
    cm("acquisition.public", "Collected from a public source", False),
    cm("acquisition.scrape", "Automated collection from a public interface", False),
    cm("inference.derived", "Derived by inference rather than collected", False),
]

# ---------------------------------------------------------------------------
# Operations
# ---------------------------------------------------------------------------
# meters: which quantity unit the operation is naturally measured in.
# derives: whether the operation can produce a new data object whose rights
#          must be propagated.
def op(code, name, family, meters, derives=False, notes=""):
    return {"code": code, "name": name, "family": family, "meter": meters,
            "derives": derives, "notes": notes}

OPERATIONS = [
    # collection
    op("collect.observe", "Collect by observation", "collect", "record"),
    op("collect.receive", "Receive from another party", "collect", "record"),
    op("collect.generate", "Generate about the subject", "collect", "record", True),
    # storage and access
    op("store.persist", "Persist to durable storage", "process", "subject_day"),
    op("store.replicate", "Replicate across systems", "process", "subject_day"),
    op("store.backup", "Retain in backup", "process", "subject_day"),
    op("access.read", "Read a record", "process", "record"),
    op("access.query", "Execute a query touching the record", "process", "query"),
    op("access.index", "Build or update an index", "process", "record", True),
    op("access.export_view", "Materialise a view or extract", "process", "record", True),
    # processing
    op("process.transform", "Transform or normalise", "process", "record", True),
    op("process.aggregate", "Aggregate with other records", "process", "record", True),
    op("process.classify", "Classify or label", "process", "inference", True),
    op("process.profile", "Build or update a profile", "process", "inference", True),
    op("process.score", "Compute a score", "process", "inference", True),
    op("process.personalize", "Personalise an experience", "process", "inference"),
    op("process.recommend", "Produce a recommendation", "process", "inference"),
    op("process.predict", "Produce a prediction", "process", "inference", True),
    op("process.match", "Match or link across datasets", "process", "record", True),
    op("process.deidentify", "De-identify or pseudonymise", "process", "record", True),
    op("process.anonymize", "Anonymise (claimed irreversible)", "process", "record", True,
       "Claim must carry an evidence reference; see PRIVACY.md on the limits."),
    op("process.dp_release", "Release under differential privacy", "process", "query", True),
    # transfer
    op("transfer.internal", "Transfer within the controller", "transfer", "record"),
    op("transfer.affiliate", "Transfer to an affiliate", "transfer", "record"),
    op("transfer.processor", "Transfer to a processor or vendor", "transfer", "record"),
    op("transfer.controller", "Transfer to an independent controller", "transfer", "record"),
    op("transfer.sale", "Sale of data", "transfer", "record"),
    op("transfer.license", "License for defined use", "transfer", "record"),
    op("transfer.api_access", "Serve through an API", "transfer", "query"),
    op("transfer.export_jurisdiction", "Cross-border transfer", "transfer", "record"),
    op("transfer.publish", "Publish or make public", "transfer", "record"),
    # AI
    op("ai.pretrain", "Include in pre-training corpus", "ai", "token", True),
    op("ai.finetune", "Include in fine-tuning set", "ai", "token", True),
    op("ai.instruction_tune", "Include in instruction-tuning set", "ai", "token", True),
    op("ai.preference_data", "Include in a preference dataset", "ai", "comparison", True),
    op("ai.rl_environment", "Use as reinforcement-learning signal", "ai", "episode", True),
    op("ai.embed", "Generate embeddings", "ai", "record", True),
    op("ai.index_retrieval", "Index for retrieval", "ai", "record", True),
    op("ai.retrieve", "Retrieve into a context window", "ai", "retrieval", False),
    op("ai.inference_input", "Use as inference input", "ai", "inference"),
    op("ai.evaluate", "Include in an evaluation set", "ai", "record", True),
    op("ai.synthesize", "Generate synthetic data from", "ai", "record", True),
    op("ai.distill", "Distil into a smaller model", "ai", "token", True),
    op("ai.unlearn", "Apply a machine-unlearning procedure", "ai", "record", True,
       "Recorded as an attempt with an evidence reference; efficacy is not assumed."),
    # commercial
    op("commercial.advertise", "Use for advertising", "commercialize", "impression"),
    op("commercial.measure", "Advertising or campaign measurement", "commercialize", "impression"),
    op("commercial.price", "Set or personalise a price", "commercialize", "inference"),
    op("commercial.underwrite", "Insurance or credit underwriting", "commercialize", "inference"),
    op("commercial.market_research", "Market research", "commercialize", "record"),
    op("commercial.product_improve", "Product improvement", "commercialize", "record"),
    op("commercial.fraud_detect", "Fraud or abuse detection", "commercialize", "inference"),
    op("commercial.resell", "Resell derived data", "commercialize", "record"),
    # lifecycle
    op("lifecycle.retain", "Continue retention past a checkpoint", "lifecycle", "subject_day"),
    op("lifecycle.delete", "Delete", "lifecycle", "record"),
    op("lifecycle.restrict", "Restrict processing", "lifecycle", "record"),
    op("lifecycle.port", "Port to the subject or a third party", "lifecycle", "record"),
]

# ---------------------------------------------------------------------------
# Purposes
# ---------------------------------------------------------------------------
# Purposes form a lattice: an authorization for a parent purpose permits its
# children. `parent: null` marks a root. Purpose limitation is enforced by
# checking reachability in this lattice, never by string matching.
def pu(code, name, parent, commercial, notes=""):
    return {"code": code, "name": name, "parent": parent,
            "commercial": commercial, "notes": notes}

PURPOSES = [
    pu("service", "Provide the requested service", None, False),
    pu("service.core", "Core service delivery", "service", False),
    pu("service.account", "Account management", "service", False),
    pu("service.support", "Customer support", "service", False),
    pu("service.personalization", "Personalisation of the service", "service", False),
    pu("security", "Security and integrity", None, False),
    pu("security.fraud", "Fraud prevention", "security", False),
    pu("security.abuse", "Abuse and safety", "security", False),
    pu("security.authentication", "Authentication", "security", False),
    pu("legal", "Legal and regulatory", None, False),
    pu("legal.obligation", "Compliance with a legal obligation", "legal", False),
    pu("legal.claims", "Establishment or defence of legal claims", "legal", False),
    pu("legal.audit", "Audit and accounting", "legal", False),
    pu("operations", "Operating the system", None, False),
    pu("operations.reliability", "Reliability and debugging", "operations", False),
    pu("operations.capacity", "Capacity planning", "operations", False),
    pu("improvement", "Improving products", None, True),
    pu("improvement.analytics", "Product analytics", "improvement", True),
    pu("improvement.experimentation", "Experimentation", "improvement", True),
    pu("improvement.ai_training", "Training machine-learning models", "improvement", True),
    pu("improvement.ai_evaluation", "Evaluating machine-learning models", "improvement", True),
    pu("marketing", "Marketing", None, True),
    pu("marketing.direct", "Direct marketing", "marketing", True),
    pu("marketing.advertising", "Advertising", "marketing", True),
    pu("marketing.advertising.contextual", "Contextual advertising",
       "marketing.advertising", True),
    pu("marketing.advertising.behavioral", "Behavioural advertising",
       "marketing.advertising", True),
    pu("marketing.measurement", "Advertising measurement", "marketing", True),
    pu("commerce", "Commercial exploitation", None, True),
    pu("commerce.sale", "Sale of data", "commerce", True),
    pu("commerce.licensing", "Licensing of data", "commerce", True),
    pu("commerce.brokerage", "Data brokerage", "commerce", True),
    pu("risk", "Risk assessment", None, True),
    pu("risk.credit", "Credit assessment", "risk", True),
    pu("risk.insurance", "Insurance underwriting", "risk", True),
    pu("risk.pricing", "Individualised pricing", "risk", True),
    pu("research", "Research", None, False),
    pu("research.scientific", "Scientific research", "research", False),
    pu("research.public_interest", "Public-interest research", "research", False),
    pu("research.commercial", "Commercial research", "research", True),
]

# ---------------------------------------------------------------------------
# Measurement units
# ---------------------------------------------------------------------------
def un(code, name, description, additive):
    return {"code": code, "name": name, "description": description,
            "additive": additive}

UNITS = [
    un("record", "record", "One data record or field group about one subject.", True),
    un("byte", "byte", "Octets of subject data at rest or in transit.", True),
    un("subject_day", "subject-day", "One subject's data retained for one day.", True),
    un("query", "query", "One query execution that touched the subject's data.", True),
    un("inference", "inference", "One inference produced using the subject's data.", True),
    un("impression", "impression", "One advertising impression informed by the data.", True),
    un("token", "token", "One training token attributable to the subject's data.", True),
    un("comparison", "comparison", "One preference comparison.", True),
    un("episode", "episode", "One reinforcement-learning episode.", True),
    un("retrieval", "retrieval", "One retrieval of the record into a context window.", True),
    un("gradient_step", "gradient-step",
       "One optimiser step in which the record appeared in the batch.", True),
    un("share", "share",
       "A dimensionless attribution share; NOT additive across scopes.", False),
]

# ---------------------------------------------------------------------------
# Jurisdictional regimes (technical tags, not legal advice)
# ---------------------------------------------------------------------------
def rg(code, name, scope, notes):
    return {"code": code, "name": name, "scope": scope, "notes": notes}

REGIMES = [
    rg("eu.gdpr", "EU General Data Protection Regulation", "territorial+targeting",
       "Regulation (EU) 2016/679."),
    rg("eu.data_act", "EU Data Act", "product+service",
       "Regulation (EU) 2023/2854; user access and sharing rights over connected-product data."),
    rg("eu.ai_act", "EU AI Act", "placing_on_market",
       "Regulation (EU) 2024/1689; training-data governance and transparency duties."),
    rg("uk.uk_gdpr", "UK GDPR and DPA 2018", "territorial+targeting", ""),
    rg("us.ca.ccpa", "California CCPA/CPRA", "state+threshold", ""),
    rg("us.co.cpa", "Colorado Privacy Act", "state+threshold", ""),
    rg("us.va.vcdpa", "Virginia CDPA", "state+threshold", ""),
    rg("us.hipaa", "HIPAA", "sectoral",
       "Applies to covered entities and business associates only."),
    rg("us.glba", "Gramm-Leach-Bliley Act", "sectoral", ""),
    rg("us.fcra", "Fair Credit Reporting Act", "sectoral", ""),
    rg("us.coppa", "COPPA", "age", ""),
    rg("us.ferpa", "FERPA", "sectoral", ""),
    rg("us.il.bipa", "Illinois Biometric Information Privacy Act", "state", ""),
    rg("br.lgpd", "Brazil LGPD", "territorial+targeting", ""),
    rg("ca.pipeda", "Canada PIPEDA", "territorial", ""),
    rg("jp.appi", "Japan APPI", "territorial", ""),
    rg("kr.pipa", "Korea PIPA", "territorial", ""),
    rg("in.dpdp", "India DPDP Act 2023", "territorial+targeting", ""),
    rg("cn.pipl", "China PIPL", "territorial+targeting", ""),
    rg("au.privacy_act", "Australia Privacy Act", "territorial", ""),
    rg("za.popia", "South Africa POPIA", "territorial", ""),
]

# Lawful-basis tags. These are *recording* categories: DUAP records what the
# controller asserts, it does not adjudicate whether the assertion is correct.
BASES = [
    {"code": "consent", "name": "Consent of the data subject"},
    {"code": "contract", "name": "Necessary for performance of a contract"},
    {"code": "legal_obligation", "name": "Necessary for a legal obligation"},
    {"code": "vital_interests", "name": "Necessary to protect vital interests"},
    {"code": "public_task", "name": "Public interest or official authority"},
    {"code": "legitimate_interests", "name": "Legitimate interests"},
    {"code": "opt_out_respected", "name": "Processing permitted absent an opt-out"},
    {"code": "protocol_authorization", "name": "DUAP authorization only",
     "notes": "The controller relies on the DUAP grant itself and asserts no other basis."},
]

ONTOLOGY = {
    "$schema": "https://duap.dev/schemas/ontology-v1.json",
    "ontology_version": "1.0.0-draft",
    "protocol": "DUAP/1",
    "notice": (
        "Regime tags and lawful-basis codes are technical recording categories. "
        "They express what a participant asserts, not a determination that the "
        "assertion is correct, and nothing here is legal advice."
    ),
    "sensitivity_tiers": TIERS,
    "data_classes": DATA_CLASSES,
    "collection_methods": COLLECTION_METHODS,
    "operations": OPERATIONS,
    "purposes": PURPOSES,
    "units": UNITS,
    "regimes": REGIMES,
    "lawful_bases": BASES,
}


def validate(o):
    errs = []
    tiers = {t["code"] for t in o["sensitivity_tiers"]}
    units = {u["code"] for u in o["units"]}
    for d in o["data_classes"]:
        if d["sensitivity"] not in tiers:
            errs.append(f"data class {d['code']}: unknown tier {d['sensitivity']}")
    for op_ in o["operations"]:
        if op_["meter"] not in units:
            errs.append(f"operation {op_['code']}: unknown unit {op_['meter']}")
    codes = {p["code"] for p in o["purposes"]}
    for p in o["purposes"]:
        if p["parent"] is not None and p["parent"] not in codes:
            errs.append(f"purpose {p['code']}: unknown parent {p['parent']}")
    for section in ("data_classes", "operations", "purposes", "units",
                    "collection_methods", "regimes", "lawful_bases",
                    "sensitivity_tiers"):
        seen = set()
        for item in o[section]:
            if item["code"] in seen:
                errs.append(f"{section}: duplicate code {item['code']}")
            seen.add(item["code"])
    return errs


if __name__ == "__main__":
    errs = validate(ONTOLOGY)
    if errs:
        print("\n".join(errs), file=sys.stderr)
        sys.exit(1)
    out = pathlib.Path(__file__).parent / "duap-ontology-v1.json"
    text = json.dumps(ONTOLOGY, indent=2, ensure_ascii=False, sort_keys=False) + "\n"
    if "--check" in sys.argv:
        if out.read_text() != text:
            print(f"{out} is out of date; run build_ontology.py", file=sys.stderr)
            sys.exit(1)
        print("ontology up to date")
    else:
        out.write_text(text)
        print(f"wrote {out} "
              f"({len(DATA_CLASSES)} classes, {len(OPERATIONS)} operations, "
              f"{len(PURPOSES)} purposes, {len(UNITS)} units, {len(REGIMES)} regimes)")
