# Literature Audit

## How to read this file

Every entry carries a **recall-confidence** annotation, because this audit was
assembled substantially from the author's knowledge of the literature rather than
from a live index of every record.

| Annotation | Meaning |
|---|---|
| **[verified]** | Confirmed against a live source during preparation of this monograph (September 2026). Venue, year, and headline finding checked. |
| **[high]** | The author is confident the work exists, of its authorship, and of its principal result. Year/volume/page may contain minor errors. |
| **[medium]** | Confident the result and the group exist; the exact venue, year, or title wording should be checked before citation in a submitted paper. |
| **[check]** | The result is real and the attribution is the author's best recollection, but the citation should be located and verified before use. |

**Evidence type** is marked **T** (theoretical/mathematical), **E** (experimental),
**C** (computational/simulation), **S** (standard/specification), **M** (market or
legal data), **R** (review).

**No entry in this file is a fabrication to the author's knowledge.** Where
confidence is below [high], that is stated rather than concealed. Any entry marked
[medium] or [check] must be located and verified before appearing in a submitted
manuscript.

---

## 1. Thermodynamics of computation and of information

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Landauer, R. (1961). "Irreversibility and heat generation in the computing process." *IBM J. Res. Dev.* 5(3):183–191. | T | [high] | §15.2 Term 3. The k_BT ln2 erasure bound. |
| Bennett, C. H. (1982). "The thermodynamics of computation—a review." *Int. J. Theor. Phys.* 21:905–940. | T,R | [high] | §15. Reversible computation; the demon analysis. |
| Bérut, A., Arakelyan, A., Petrosyan, A., Ciliberto, S., Dillenschneider, R., Lutz, E. (2012). "Experimental verification of Landauer's principle linking information and thermodynamics." *Nature* 483:187–189. | E | [high] | §15.2. Experimental confirmation at the bound. |
| Jun, Y., Gavrilov, M., Bechhoefer, J. (2014). "High-precision test of Landauer's principle in a feedback trap." *Phys. Rev. Lett.* 113:190601. | E | [medium] | §15.2. Tighter experimental test. |
| Jarzynski, C. (1997). "Nonequilibrium equality for free energy differences." *Phys. Rev. Lett.* 78:2690. | T | [high] | §15. Work–free-energy relation for finite-time processes. |
| Crooks, G. E. (1999). "Entropy production fluctuation theorem and the nonequilibrium work relation." *Phys. Rev. E* 60:2721. | T | [high] | §15. |
| Sagawa, T., Ueda, M. (2010). "Generalized Jarzynski equality under nonequilibrium feedback control." *Phys. Rev. Lett.* 104:090602. | T | [medium] | §15.2 Term 4. Measurement/feedback thermodynamics. |
| Parrondo, J. M. R., Horowitz, J. M., Sagawa, T. (2015). "Thermodynamics of information." *Nature Physics* 11:131–139. | R | [high] | §15. The best single review for the framework's purposes. |
| Sivak, D. A., Crooks, G. E. (2012). "Thermodynamic metrics and optimal paths." *Phys. Rev. Lett.* 108:190602. | T | [high] | §15.2. Thermodynamic length; W_ex ≈ 𝓛²/2τ. |
| Aurell, E., Mejía-Monasterio, C., Muratore-Ginanneschi, P. (2011). "Optimal protocols and optimal transport in stochastic thermodynamics." *Phys. Rev. Lett.* 106:250601. | T | [high] | §15.2. Wasserstein-2 bound on dissipated work. |
| Barato, A. C., Seifert, U. (2015). "Thermodynamic uncertainty relation for biomolecular processes." *Phys. Rev. Lett.* 114:158101. | T | [high] | §15.2, E5. Precision costs dissipation. |
| Gingrich, T. R., Horowitz, J. M., Perunov, N., England, J. L. (2016). "Dissipation bounds all steady-state current fluctuations." *Phys. Rev. Lett.* 116:120601. | T | [high] | §15.2. Proof of the TUR. |
| Shiraishi, N., Funo, K., Saito, K. (2018). "Speed limit for classical stochastic processes." *Phys. Rev. Lett.* 121:070601. | T | [medium] | §15.2. Classical speed limit. |
| Mandelstam, L., Tamm, I. (1945). *J. Phys. USSR* 9:249. | T | [high] | §15.2. Energy–time uncertainty speed limit. |
| Margolus, N., Levitin, L. B. (1998). "The maximum speed of dynamical evolution." *Physica D* 120:188–195. | T | [high] | §15.2. Quantum speed limit from mean energy. |
| Horodecki, M., Oppenheim, J. (2013). "Fundamental limitations for quantum and nanoscale thermodynamics." *Nature Communications* 4:2059. | T | [high] | §3.2. Thermomajorization; resource theory of thermal operations. |
| Brandão, F., Horodecki, M., Ng, N., Oppenheim, J., Wehner, S. (2015). "The second laws of quantum thermodynamics." *PNAS* 112:3275–3279. | T | [high] | §3.2. A family of second laws in the single-shot regime. |

## 2. Complexity theory

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Kitaev, A. Yu., Shen, A., Vyalyi, M. (2002). *Classical and Quantum Computation.* AMS. | T | [high] | §16.5. QMA-completeness of 5-local Hamiltonian; the founding result. |
| Kempe, J., Kitaev, A., Regev, O. (2006). "The complexity of the local Hamiltonian problem." *SIAM J. Comput.* 35(5):1070–1097. | T | [high] | §16.5. 2-local Hamiltonian is QMA-complete. |
| Oliveira, R., Terhal, B. M. (2008). "The complexity of quantum spin systems on a two-dimensional square lattice." *Quantum Inf. Comput.* 8:900. | T | [high] | §16.5. Hardness survives realistic 2D geometry. |
| Cubitt, T., Montanaro, A. (2016). "Complexity classification of local Hamiltonian problems." *SIAM J. Comput.* 45(2):268–316. | T | [high] | §16.5. Full classification of 2-local qubit Hamiltonians — the nearest existing thing to the proposal's PCC taxonomy. |
| Cubitt, T., Montanaro, A., Piddock, S. (2018). "Universal quantum Hamiltonians." *PNAS* 115(38):9497–9502. | T | [high] | §17.4. The one genuine universality theorem in the framework. |
| Cubitt, T., Pérez-García, D., Wolf, M. M. (2015). "Undecidability of the spectral gap." *Nature* 528:207–211. | T | [high] | §16.2. |
| Barahona, F. (1982). "On the computational complexity of Ising spin glass models." *J. Phys. A* 15:3241. | T | [high] | §16.4. Ising ground state NP-complete. |
| Berger, R. (1966). "The undecidability of the domino problem." *Memoirs of the AMS* 66. | T | [high] | §16.2, §18.1. Wang tiles. |
| Reif, J. (1979). "Complexity of the mover's problem and generalizations." *FOCS*. | T | [high] | §16.4. Motion planning PSPACE-hard. |
| Hopcroft, J., Schwartz, J., Sharir, M. (1984). "On the complexity of motion planning for multiple independent objects; PSPACE-hardness of the warehouseman's problem." *Int. J. Robotics Research* 3(4):76–88. | T | [high] | §16.4. |
| Henzinger, T., Kopke, P., Puri, A., Varaiya, P. (1995/1998). "What's decidable about hybrid automata?" *STOC 1995*; *J. Comput. Syst. Sci.* 57:94–124. | T | [high] | §16.2. The undecidability boundary for hybrid reachability. |
| Czerwiński, W., Orlikowski, Ł. (2021). "Reachability in vector addition systems is Ackermann-complete." *FOCS 2021*. | T | **[verified]** | §16.3. Lower bound. |
| Leroux, J. (2021). "The reachability problem for Petri nets is not primitive recursive." *FOCS 2021*. | T | **[verified]** | §16.3. |
| Leroux, J., Schmitz, S. (2019). "Reachability in vector addition systems is primitive-recursive in fixed dimension" / Ackermannian upper bound. | T | [medium] | §16.3. Upper bound side. |
| Berger, B., Leighton, T. (1998). "Protein folding in the hydrophobic-hydrophilic (HP) model is NP-complete." *J. Comput. Biol.* 5(1):27–40. | T | [high] | §16.4. |
| Bylander, T. (1994). "The computational complexity of propositional STRIPS planning." *Artificial Intelligence* 69:165–204. | T | [high] | §16.4. PSPACE-completeness of planning. |
| Helmert, M. (2002). "Decidability and undecidability results for planning with numerical state variables." *AIPS*. | T | [medium] | §16.4. Undecidability with numeric fluents. |
| Nemhauser, G., Wolsey, L., Fisher, M. (1978). "An analysis of approximations for maximizing submodular set functions." *Math. Prog.* 14:265–294. | T | [high] | §16.4. The (1−1/e) greedy guarantee used for experiment selection. |
| Wolpert, D., Macready, W. (1997). "No free lunch theorems for optimization." *IEEE Trans. Evol. Comput.* 1(1):67–82. | T | [high] | §17.3. |
| Adleman, L., Cheng, Q., Goel, A., Huang, M.-D. (2001). "Running time and program size for self-assembled squares." *STOC*. | T | [high] | §18.1. Θ(log N/log log N) tile complexity. |
| Rothemund, P., Winfree, E. (2000). "The program-size complexity of self-assembled squares." *STOC*. | T | [high] | §18.1. |
| Doty, D., Lutz, J., Patitz, M., Schweller, R., Summers, S., Woods, D. (2012). "The tile assembly model is intrinsically universal." *FOCS*. | T | [high] | §18.1. |

## 3. Quantum information, control, and hardware

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Feynman, R. (1982). "Simulating physics with computers." *Int. J. Theor. Phys.* 21:467. | T | [high] | §8.3 Case A. |
| Lloyd, S. (1996). "Universal quantum simulators." *Science* 273:1073. | T | [high] | §8.3, §16.5. |
| Aharonov, D., van Dam, W., Kempe, J., Landau, Z., Lloyd, S., Regev, O. (2007). "Adiabatic quantum computation is equivalent to standard quantum computation." *SIAM J. Comput.* 37(1):166–194. | T | [high] | §8.4. Universality of AQC ≠ speedup. |
| Verstraete, F., Wolf, M. M., Cirac, J. I. (2009). "Quantum computation and quantum-state engineering driven by dissipation." *Nature Physics* 5:633–636. | T | [high] | §8.4, §9.7. Engineered dissipation as a universal resource — the framework's key "attractor engineering" citation. |
| Khaneja, N., Reiss, T., Kehlet, C., Schulte-Herbrüggen, T., Glaser, S. (2005). "Optimal control of coupled spin dynamics: GRAPE." *J. Magn. Reson.* 172:296–305. | T,C | [high] | §6.6, §8.4. |
| Caneva, T., Calarco, T., Montangero, S. (2011). "Chopped random-basis quantum optimization." *Phys. Rev. A* 84:022326. | T,C | [high] | §8.4. CRAB. |
| Jurdjevic, V., Sussmann, H. (1972). "Control systems on Lie groups." *J. Differential Equations* 12:313–329. | T | [high] | §7.3, §17.4. Lie algebra rank condition. |
| Ramakrishna, V., Salapaka, M., Dahleh, M., Rabitz, H., Peirce, A. (1995). "Controllability of molecular systems." *Phys. Rev. A* 51:960. | T | [high] | §7.3. LARC for quantum systems. |
| Lidar, D., Chuang, I., Whaley, K. B. (1998). "Decoherence-free subspaces for quantum computation." *Phys. Rev. Lett.* 81:2594. | T | [high] | §9.1(a). |
| Viola, L., Knill, E., Lloyd, S. (1999). "Dynamical decoupling of open quantum systems." *Phys. Rev. Lett.* 82:2417. | T | [high] | §9.1(d). |
| Google Quantum AI (2024). "Quantum error correction below the surface code threshold." *Nature* 638(8052):920–926. | E | **[verified]** | §9.4. Distance-7, 101 qubits, 0.143%±0.003% error/cycle, Λ=2.14±0.02, real-time decoding at 63 µs. |
| Arute, F. et al. (2019). "Quantum supremacy using a programmable superconducting processor." *Nature* 574:505–510. | E | [high] | §8.3. And the subsequent classical-simulation erosion. |
| Rønnow, T. F. et al. (2014). "Defining and detecting quantum speedup." *Science* 345:420–424. | C,E | [high] | §8.3 Case B, F-8. The methodological standard for speedup claims. |
| Reiher, M., Wiebe, N., Svore, K., Wecker, D., Troyer, M. (2017). "Elucidating reaction mechanisms on quantum computers." *PNAS* 114(29):7555–7560. | T,C | [high] | §9.3. FeMoco resource estimate. |
| Lee, S., Lee, J., Zhai, H., Tong, Y., Dalzell, A., Kumar, A., Helms, P., Gray, J., Cui, Z.-H., Liu, W., Kastoryano, M., Babbush, R., Preskill, J., Reichman, D., Campbell, E., Valeev, E., Lin, L., Chan, G. K.-L. (2023). "Evaluating the evidence for exponential quantum advantage in ground-state quantum chemistry." *Nature Communications* 14:1952. | T,C | **[verified]** | §9.3. Evidence for exponential advantage across chemical space has not been found. |
| Haah, J., Harrow, A., Ji, Z., Wu, X., Yu, N. (2017). "Sample-optimal tomography of quantum states." *IEEE Trans. Inf. Theory* 63(9):5628–5641 (STOC 2016). | T | [high] | §11.2. Θ(d²/ε²). |
| O'Donnell, R., Wright, J. (2016). "Efficient quantum tomography." *STOC*. | T | [high] | §11.2. |
| Huang, H.-Y., Kueng, R., Preskill, J. (2020). "Predicting many properties of a quantum system from very few measurements." *Nature Physics* 16:1050–1057. | T | [high] | §11.2. Classical shadows: O(log m/ε²). |
| Aaronson, S. (2018). "Shadow tomography of quantum states." *STOC*. | T | [high] | §11.2. |
| Flammia, S., Liu, Y.-K. (2011). "Direct fidelity estimation from few Pauli measurements." *Phys. Rev. Lett.* 106:230501. | T | [high] | §11.2. |
| Bar-Gill, N., Pham, L., Jarmola, A., Budker, D., Walsworth, R. (2013). "Solid-state electronic spin coherence time approaching one second." *Nature Communications* 4:1743. | E | [high] | §9.5. NV coherence (low temperature). |
| Wang, P. et al. (2021). "Single ion qubit with estimated coherence time exceeding one hour." *Nature Communications* 12:233. | E | [medium] | §9.5. |
| Ebadi, S. et al. (2021). "Quantum phases of matter on a 256-atom programmable quantum simulator." *Nature* 595:227–232. | E | [high] | §9.5. Neutral-atom scale. |
| Bluvstein, D. et al. (2024). "Logical quantum processor based on reconfigurable atom arrays." *Nature* 626:58–65. | E | [high] | §9.5. |
| Lescanne, R. et al. (2020). "Exponential suppression of bit-flips in a qubit encoded in an oscillator." *Nature Physics* 16:509–513. | E | [medium] | §9.1(c). Dissipatively stabilized cat qubits. |
| Šuntajs, J., Bonča, J., Prosen, T., Vidmar, L. (2020). "Quantum chaos challenges many-body localization." *Phys. Rev. E* 102:062144. | C | [medium] | §9.5. MBL contested. |
| De Roeck, W., Huveneers, F. (2017). "Stability and instability towards delocalization in many-body localization systems." *Phys. Rev. B* 95:155129. | T | [medium] | §9.5. Avalanche instability. |
| Duan, H.-G. et al. (2017). "Nature does not rely on long-lived electronic quantum coherence for photosynthetic energy transfer." *PNAS* 114(32):8493–8498. | E | [high] | §9.2. Deflation of the quantum-biology computation claim. |
| Hore, P. J., Mouritsen, H. (2016). "The radical-pair mechanism of magnetoreception." *Annu. Rev. Biophys.* 45:299–344. | R | [high] | §9.2. The surviving quantum-biology hypothesis (sensing, not computing). |

## 4. Self-assembly and programmable matter

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Winfree, E. (1998). *Algorithmic Self-Assembly of DNA.* PhD thesis, Caltech. | T | [high] | §18.1. The abstract Tile Assembly Model. |
| Rothemund, P. W. K. (2006). "Folding DNA to create nanoscale shapes and patterns." *Nature* 440:297–302. | E | [high] | §18.1. DNA origami. |
| Rothemund, P., Papadakis, N., Winfree, E. (2004). "Algorithmic self-assembly of DNA Sierpinski triangles." *PLoS Biology* 2(12):e424. | E | [high] | §18.1. |
| Ke, Y., Ong, L., Shih, W., Yin, P. (2012). "Three-dimensional structures self-assembled from DNA bricks." *Science* 338:1177–1183. | E | [high] | §18.1. |
| Woods, D., Doty, D., Myhrvold, C., Hui, J., Zhou, F., Yin, P., Winfree, E. (2019). "Diverse and robust molecular algorithms using reprogrammable DNA self-assembly." *Nature* 567:366–372. | E | [high] | §18.1. 21 algorithms from one reprogrammable tile set. |
| Tikhomirov, G., Petersen, P., Qian, L. (2017). "Fractal assembly of micrometre-scale DNA origami arrays with arbitrary patterns." *Nature* 552:67–71. | E | [high] | §18.1. Hierarchical assembly. |
| Douglas, S. et al. (2009). "Rapid prototyping of 3D DNA-origami shapes with caDNAno." *Nucleic Acids Research* 37:5001–5006. | C | [high] | §18.1. A real, deployed reality compiler. |
| Winfree, E., Bekbolatov, R. (2004). "Proofreading tile sets: error correction for algorithmic self-assembly." *DNA Computing*. | T | [medium] | §18.2. Physical error correction. |
| Rubenstein, M., Cornejo, A., Nagpal, R. (2014). "Programmable self-assembly in a thousand-robot swarm." *Science* 345:795–799. | E | [high] | §18.4. Macroscale local-rule compilation. |
| Turing, A. M. (1952). "The chemical basis of morphogenesis." *Phil. Trans. R. Soc. B* 237:37–72. | T | [high] | §8.4, §19.2. |

## 5. Manufacturing, atomically precise fabrication, metrology

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Eigler, D. M., Schweizer, E. K. (1990). "Positioning single atoms with a scanning tunnelling microscope." *Nature* 344:524–526. | E | [high] | §10.1. |
| Ho, W., Lee, H. J. (1999). "Single bond formation and characterization with a scanning tunneling microscope." *Science* 286:1719–1722. | E | [high] | §10.1. Positional chemistry is possible. |
| Fuechsle, M. et al. (2012). "A single-atom transistor." *Nature Nanotechnology* 7:242–246. | E | [high] | §9.3, §10.1. Deterministic single-dopant device. |
| Randall, J. N. et al. — hydrogen depassivation lithography for atomically precise patterning, *J. Vac. Sci. Technol. B* (various, 2009–2020). | E | [check] | §10.1. Exact citation to be located; the technique and the group are real. |
| "Nanotechnology: Drexler and Smalley make the case for and against 'molecular assemblers'." *Chemical & Engineering News* 81(48), 1 Dec 2003. | R | [high] | §10.1. Fat/sticky fingers objections. |
| Moriarty, P. et al. — experimental work on mechanosynthesis and critiques of assembler proposals. | E | [check] | §10.1. Real body of work; specific citation to be located. |
| ASME Y14.5-2018, *Dimensioning and Tolerancing*. | S | [high] | §4.4, §5.2. GD&T as an acceptance-region formalism. |
| ASTM E8/E8M, *Standard Test Methods for Tension Testing of Metallic Materials*. | S | [high] | §5.3. |
| ASTM E691, *Conducting an Interlaboratory Study to Determine the Precision of a Test Method*; ISO 5725 series. | S | [high] | §25 F-7. The round-robin protocol. |
| ISO/IEC 17025:2017, *General requirements for the competence of testing and calibration laboratories*. | S | [high] | §14.3, §23.2. Independence of verification. |
| IEC 61511, *Functional safety — safety instrumented systems for the process industry*. | S | [high] | §6.8, §13.4. |
| ICH Q3C, *Impurities: Guideline for Residual Solvents*. | S | [high] | §5.4. |
| FDA (2004). *Guidance for Industry: PAT — A Framework for Innovative Pharmaceutical Development, Manufacturing, and Quality Assurance.* | S | [high] | §10.4. Regulatory precedent for in-process verification. |

## 6. Materials informatics, inverse design, autonomous laboratories

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Jain, A. et al. (2013). "Commentary: The Materials Project." *APL Materials* 1:011002. | C | [high] | §20.3. |
| Merchant, A., Batzner, S., Schoenholz, S., Aykol, M., Cheon, G., Cubuk, E. D. (2023). "Scaling deep learning for materials discovery." *Nature* 624:80–85. | C | **[verified]** | §20.3. GNoME: 2.2M structures, ~381k predicted stable. |
| Cheetham, A. K., Seshadri, R. (2024). "Artificial intelligence driving materials discovery? Perspective on the article: Scaling deep learning for materials discovery." *Chemistry of Materials.* | R | **[verified]** | §20.3. The published critique of the above. |
| Szymanski, N. J. et al. (2023). "An autonomous laboratory for the accelerated synthesis of novel materials." *Nature* 624:86–91. | E | **[verified]** | §20.3. A-Lab: 41/58 targets in 17 days. |
| Leeman, J., Liu, Y., Stiles, J., Lee, S. B., Bhatt, P., Schoop, L. M., Palgrave, R. G. (2024). "Challenges in high-throughput inorganic materials prediction and autonomous synthesis." *PRX Energy* 3:011002. | R | **[verified]** | §20.3, §21 Stage 2. Argues, on re-examination of all 43 reported products, that no new materials were demonstrated — the characterization-layer failure. |
| Batzner, S. et al. (2022). "E(3)-equivariant graph neural networks for data-efficient and accurate interatomic potentials." *Nature Communications* 13:2453. | C | [high] | §20.3. NequIP. |
| Batatia, I., Kovács, D. P., Simm, G., Ortner, C., Csányi, G. (2022). "MACE: Higher order equivariant message passing neural networks for fast and accurate force fields." *NeurIPS*. | C | [high] | §20.3. |
| Chen, C., Ong, S. P. (2022). "A universal graph deep learning interatomic potential for the periodic table." *Nature Computational Science* 2:718–728. | C | [high] | §20.3. M3GNet. |
| Deng, B. et al. (2023). "CHGNet as a pretrained universal neural network potential for charge-informed atomistic modelling." *Nature Machine Intelligence* 5:1031–1041. | C | [medium] | §20.3. |
| Molesky, S., Lin, Z., Piggott, A., Jin, W., Vučković, J., Rodriguez, A. (2018). "Inverse design in nanophotonics." *Nature Photonics* 12:659–670. | R | [high] | §20.3. The working reality compiler in photonics. |
| Piggott, A. et al. (2015). "Inverse design and demonstration of a compact and broadband on-chip wavelength demultiplexer." *Nature Photonics* 9:374–377. | E | [high] | §20.3. |
| MacLeod, B. P. et al. (2020). "Self-driving laboratory for accelerated discovery of thin-film materials." *Science Advances* 6:eaaz8867. | E | [high] | §21 Stage 2. |
| Burger, B. et al. (2020). "A mobile robotic chemist." *Nature* 583:237–241. | E | [high] | §21 Stage 2. |
| Steiner, S. et al. (2019). "Organic synthesis in a modular robotic system driven by a chemical programming language." *Science* 363:eaav2211. | E | [high] | §5, §10.2. χDL — a real chemical DSL; direct prior art for the Reality DSL. |
| Segler, M., Preuss, M., Waller, M. (2018). "Planning chemical syntheses with deep neural networks and symbolic AI." *Nature* 555:604–610. | C | [high] | §6.5. |
| Coley, C. W. et al. (2019). "A robotic platform for flow synthesis of organic compounds informed by AI planning." *Science* 365:eaax1566. | E | [high] | §6.5, §10.2. |
| Lindley, D. V. (1956). "On a measure of the information provided by an experiment." *Ann. Math. Stat.* 27:986–1005; Chaloner, K., Verdinelli, I. (1995). "Bayesian experimental design: a review." *Statistical Science* 10:273–304. | T,R | [high] | §6.9. Optimal experimental design. |

## 7. Biology, morphogenesis, protein design

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| Wolpert, L. (1969). "Positional information and the spatial pattern of cellular differentiation." *J. Theor. Biol.* 25:1–47. | T | [high] | §19.2. |
| Raspopovic, J., Marcon, L., Russo, L., Sharpe, J. (2014). "Digit patterning is controlled by a Bmp-Sox9-Wnt Turing network." *Science* 345:566–570. | E,C | [high] | §19.2. A Turing mechanism in real development. |
| Sheth, R. et al. (2012). "Hox genes regulate digit patterning by controlling the wavelength of a Turing-type mechanism." *Science* 338:1476–1480. | E | [medium] | §19.2. |
| Toda, S., Blauch, L., Tang, S., Morsut, L., Lim, W. (2018). "Programming self-organizing multicellular structures with synthetic cell-cell signaling." *Science* 361:156–162. | E | [high] | §19.2. Local rules → target morphology, synthetically. |
| Kriegman, S., Blackiston, D., Levin, M., Bongard, J. (2020). "A scalable pipeline for designing reconfigurable organisms." *PNAS* 117(4):1853–1859. | C,E | [high] | §19.2. Design → biological realization loop closed. |
| Levin, M. — bioelectric patterning; multiple papers 2012–2023, e.g. *Cell* (2021) review on bioelectric signaling. | E,R | [medium] | §19.2. Phenomenon well replicated; mechanistic interpretation still developing. |
| Jumper, J. et al. (2021). "Highly accurate protein structure prediction with AlphaFold." *Nature* 596:583–589. | C | [high] | §18.4, §19.2. |
| Watson, J. L. et al. (2023). "De novo design of protein structure and function with RFdiffusion." *Nature* 620:1089–1100. | C,E | [high] | §19.2. The working biological reality compiler. |

## 8. Cryptography, provenance, and security

| Work | Type | Conf. | Relevance |
|---|---|---|---|
| NIST FIPS 202 (2015). *SHA-3 Standard: Permutation-Based Hash and Extendable-Output Functions.* | S | [high] | §12.4. |
| NIST FIPS 203 / 204 / 205 (August 2024). *ML-KEM / ML-DSA / SLH-DSA.* | S | [high] | §12.4. Post-quantum standards. |
| Laurie, B., Langley, A., Kasper, E. (2013). *Certificate Transparency.* RFC 6962. | S | [high] | §12.4, §13.4. The architecture to copy. |
| Torres-Arias, S., Afzali, H., Kuppusamy, T. K., Curtmola, R., Cappos, J. (2019). "in-toto: Providing farm-to-table guarantees for bits and bytes." *USENIX Security*. | S,T | [high] | §12.4. Supply-chain attestation, directly transferable. |
| OpenSSF. *SLSA: Supply-chain Levels for Software Artifacts.* | S | [high] | §12.4. |
| Newman, Z., Meyers, J. S., Torres-Arias, S. (2022). "Sigstore: Software signing for everybody." *ACM CCS*. | S | [medium] | §12.4. |
| Pappu, R., Recht, B., Taylor, J., Gershenfeld, N. (2002). "Physical one-way functions." *Science* 297:2026–2030. | T,E | [high] | §12.3. The founding PUF paper. |
| Rührmair, U. et al. (2010). "Modeling attacks on physical unclonable functions." *ACM CCS*. | T,E | [high] | §12.3. Why PUF-1 must be stated as an assumption with a bound. |
| Buchanan, J. D. R. et al. (2005). "Fingerprinting documents and packaging." *Nature* 436:475. | E | [medium] | §12.3. Intrinsic surface fingerprinting. |
| Trusted Computing Group. *TPM 2.0 Library Specification*; *DICE* specifications. | S | [high] | §12.1, §14.3. |

## 9. Legal and market references

| Source | Type | Conf. | Relevance |
|---|---|---|---|
| *Alice Corp. v. CLS Bank International*, 573 U.S. 208 (2014). | M | [high] | §24.1. Abstract-idea exclusion, two-step framework. |
| *Mayo Collaborative Services v. Prometheus Laboratories*, 566 U.S. 66 (2012). | M | [high] | §24.1. Laws of nature. |
| *Diamond v. Diehr*, 450 U.S. 175 (1981). | M | [high] | §24.2. A physical process using an equation is eligible — the model claim shape. |
| EPO Enlarged Board of Appeal, **G 1/19** (2021), *Pedestrian simulation*. | M | [high] | §24.2. Computer-implemented simulations and technical effect. |
| *Thaler v. Vidal*, 43 F.4th 1207 (Fed. Cir. 2022). | M | [medium] | §24.3. An AI cannot be a named inventor under US law. |
| Semiconductor Industry Association / WSTS: global semiconductor sales $627.6B in 2024 (+19.1%); Gartner: $655.9B (+21%). | M | **[verified]** | §22.2. |
| EDA market size 2024: ~$15–17B (multiple market-research estimates; ranges vary by scope). | M | **[verified]** | §22.2. Capture ratio ≈ 2.4–2.7%. |
| Testing, Inspection & Certification (TIC) global market: ~$230–270B. | M | [medium] | §22.2. Figure is approximate and scope-dependent; verify before use in a funding document. |
| World Bank: global manufacturing value added ≈ $16T; world GDP ≈ $105T (2024). | M | [medium] | §22.1, §22.3. Approximate; verify current vintage. |

---

## Gaps in this audit

Stated explicitly, because an audit that claims completeness is not an audit.

1. **No systematic search was performed** across the seventeen fields the brief
   names. This is a domain-expert reading list, not a bibliometric review. A genuine
   systematic review — with search strings, databases, inclusion criteria, and PRISMA-
   style accounting — is a separate 3–6 month effort and should be commissioned before
   any grant submission.
2. **Market figures are the weakest entries.** They come from commercial
   market-research estimates whose methodologies differ and whose scopes overlap
   inconsistently. Treat every dollar figure in §22 as order-of-magnitude.
3. **Fast-moving areas will have moved.** Quantum hardware numbers (§9.5), MLIP
   accuracy, and protein-design hit rates change on a timescale of months. Every
   number in the feasibility matrix should be re-checked at time of use.
4. **Several important literatures are under-represented here** relative to their
   importance to the framework: finite-time thermodynamics of driven many-body
   systems; formal methods for cyber-physical systems (hybrid-systems verification
   tools such as reachability libraries); acceptance sampling and reliability
   statistics; and the industrial literature on process qualification. These should be
   the first additions.
