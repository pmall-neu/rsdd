//! Contains all randomized and integration tests for the RSDD library
//! Many of these are property based tests that are created using QuickCheck

extern crate rsdd;
#[macro_use]
extern crate quickcheck;
use crate::repr::cnf::Cnf;
use crate::repr::var_label::VarLabel;
use rsdd::builder::bdd_builder::BddManager;
use rsdd::builder::cache::all_app::AllTable;
use rsdd::builder::sdd_builder::{CompressionSddManager, SddBuilder};
use rsdd::repr::robdd::BddPtr;
use rsdd::repr::vtree::VTree;
use rsdd::*;
extern crate rand;

/// a prime large enough to ensure no collisions for semantic hashing
const BIG_PRIME: u128 = 100000049;

/// A list of canonical forms in DIMACS form. The goal of these tests is to ensure that caching
/// and application are working as intended
static C1_A: &str = "
p cnf 5 3
1 2 0
-1 2 0
";

static C1_B: &str = "
p cnf 2 1
2 0
";

static C2_A: &str = "
p cnf 5 3
1 2 3 0
1 2 0
-1 2 0
";

static C2_B: &str = "
p cnf 2 1
2 0
";

static C3_A: &str = "
p cnf 5 3
1 2 3 4 5 0
1 2 0
-1 2 0
";

static C3_B: &str = "
p cnf 2 1
2 0
";

static C4_A: &str = "
p cnf 5 3
-1 2 0
1
";

static C4_B: &str = "
p cnf 2 1
1 0
2 0
";

static C5_A: &str = "
p cnf 5 3
-1 2 0
-2 3 0
-3 4 0
-4 5 0
-5 6 0
-6 7 0
-7 8 0
1
";

static C5_B: &str = "
p cnf 2 1
1 0
2 0
3 0
4 0
5 0
6 0
7 0
8 0
";

static C6_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
1 8 0
";

static C6_B: &str = "
p cnf 2 1
1 8 0
";

static C7_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
3 5 0
";

static C7_B: &str = "
p cnf 2 1
3 5 0
";

static C8_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
1 5 0
";

static C8_B: &str = "
p cnf 2 1
1 5 0
";

static C9_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
2 6 0
";

static C9_B: &str = "
p cnf 2 1
2 6 0
";

static C10_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
2 7 0
";

static C10_B: &str = "
p cnf 2 1
2 7 0
";

static C11_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
1 3 5 0
";

static C11_B: &str = "
p cnf 2 1
1 3 5 0
";

static C12_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
2 4 8 0
";

static C12_B: &str = "
p cnf 2 1
2 4 8 0
";

static C13_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
1 2 3 4 5 6 7 0
";

static C13_B: &str = "
p cnf 2 1
1 2 3 4 5 6 7 0
";

static C14_A: &str = "
p cnf 8 3
1 2 3 4 5 6 7 8 0
2 3 4 5 6 7 8 0
";

static C14_B: &str = "
p cnf 2 1
2 3 4 5 6 7 8 0
";

fn get_canonical_forms() -> Vec<(Cnf, Cnf)> {
    vec![
        (
            Cnf::from_file(String::from(C1_A)),
            Cnf::from_file(String::from(C1_B)),
        ),
        (
            Cnf::from_file(String::from(C2_A)),
            Cnf::from_file(String::from(C2_B)),
        ),
        (
            Cnf::from_file(String::from(C3_A)),
            Cnf::from_file(String::from(C3_B)),
        ),
        (
            Cnf::from_file(String::from(C4_A)),
            Cnf::from_file(String::from(C4_B)),
        ),
        (
            Cnf::from_file(String::from(C5_A)),
            Cnf::from_file(String::from(C5_B)),
        ),
        (
            Cnf::from_file(String::from(C6_A)),
            Cnf::from_file(String::from(C6_B)),
        ),
        (
            Cnf::from_file(String::from(C7_A)),
            Cnf::from_file(String::from(C7_B)),
        ),
        (
            Cnf::from_file(String::from(C8_A)),
            Cnf::from_file(String::from(C8_B)),
        ),
        (
            Cnf::from_file(String::from(C9_A)),
            Cnf::from_file(String::from(C9_B)),
        ),
        (
            Cnf::from_file(String::from(C10_A)),
            Cnf::from_file(String::from(C10_B)),
        ),
        (
            Cnf::from_file(String::from(C11_A)),
            Cnf::from_file(String::from(C11_B)),
        ),
        (
            Cnf::from_file(String::from(C12_A)),
            Cnf::from_file(String::from(C12_B)),
        ),
        (
            Cnf::from_file(String::from(C13_A)),
            Cnf::from_file(String::from(C13_B)),
        ),
        (
            Cnf::from_file(String::from(C14_A)),
            Cnf::from_file(String::from(C14_B)),
        ),
    ]
}

#[test]
fn test_bdd_canonicity() {
    for (cnf1, cnf2) in get_canonical_forms().into_iter() {
        let man = BddManager::<AllTable<BddPtr>>::new_default_order(cnf1.num_vars());
        let r1 = man.from_cnf(&cnf1);
        let r2 = man.from_cnf(&cnf2);
        assert!(
            builder::bdd_builder::BddBuilder::eq_bdd(&man, r1, r2),
            "Not eq\nCNF 1: {:?}\nCNF 2: {:?}\nBDD 1:{}\n BDD 2: {}",
            cnf1,
            cnf2,
            r1.to_string_debug(),
            r2.to_string_debug()
        );
    }
}

#[test]
fn test_sdd_canonicity() {
    for (cnf1, cnf2) in get_canonical_forms().into_iter() {
        let v: Vec<VarLabel> = (0..cnf1.num_vars())
            .map(|x| VarLabel::new(x as u64))
            .collect();
        let vtree = VTree::even_split(&v, 1);
        let man = CompressionSddManager::new(vtree);
        let r1 = man.from_cnf(&cnf1);
        let r2 = man.from_cnf(&cnf2);
        assert!(
            man.sdd_eq(r1, r2),
            "Not eq\nCNF 1: {:?}\nCNF 2: {:?}\nSDD 1:{}\n SDD 2: {}",
            cnf1,
            cnf2,
            man.print_sdd(r1),
            man.print_sdd(r2)
        );
    }
}

#[test]
fn test_sdd_is_canonical() {
    for (cnf1, cnf2) in get_canonical_forms().into_iter() {
        let v: Vec<VarLabel> = (0..cnf1.num_vars())
            .map(|x| VarLabel::new(x as u64))
            .collect();
        let vtree = VTree::even_split(&v, 1);
        let man = CompressionSddManager::new(vtree);
        let r1 = man.from_cnf(&cnf1);
        let r2 = man.from_cnf(&cnf2);
        assert!(
            r1.is_canonical(),
            "Not canonical\nCNF 1: {:?}\nSDD 1:{}",
            cnf1,
            man.print_sdd(r1),
        );
        assert!(
            r2.is_canonical(),
            "Not canonical\nCNF 2: {:?}\nSDD 2:{}",
            cnf2,
            man.print_sdd(r2),
        );
    }
}

#[cfg(test)]
mod test_bdd_manager {
    use crate::builder::decision_nnf_builder::DecisionNNFBuilder;
    use crate::repr::cnf::Cnf;
    use crate::repr::var_label::VarLabel;
    use crate::rsdd::builder::BottomUpBuilder;
    use quickcheck::TestResult;
    use rand::Rng;
    use rsdd::builder::cache::all_app::AllTable;
    use rsdd::builder::cache::lru_app::BddApplyTable;
    use rsdd::builder::sdd_builder::SddBuilder;
    use rsdd::repr::ddnnf::{create_semantic_hash_map, DDNNFPtr};
    use rsdd::repr::dtree::DTree;
    use rsdd::repr::model::PartialModel;
    use rsdd::repr::robdd::BddPtr;
    use rsdd::repr::var_order::VarOrder;
    use rsdd::repr::vtree::VTree;
    use rsdd::repr::wmc::WmcParams;
    use rsdd::util::semirings::expectation::ExpectedUtility;
    use rsdd::util::semirings::realsemiring::RealSemiring;
    use rsdd::util::semirings::semiring_traits::Semiring;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    quickcheck! {
        fn test_cond_and(c: Cnf) -> bool {
            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(16);
            let cnf = mgr.from_cnf(&c);
            let v1 = VarLabel::new(0);
            let bdd1 = mgr.exists(cnf, v1);

            let bdd2 = mgr.condition(cnf, v1, true);
            let bdd3 = mgr.condition(cnf, v1, false);
            let bdd4 = mgr.or(bdd2, bdd3);
            bdd4 == bdd1
        }
    }

    quickcheck! {
        fn test_ite_and(c1: Cnf, c2: Cnf) -> bool {
            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(16);
            let cnf1 = mgr.from_cnf(&c1);
            let cnf2 = mgr.from_cnf(&c2);

            let itebdd = mgr.ite(cnf1, cnf2, BddPtr::false_ptr());
            let andbdd = mgr.and(cnf1, cnf2);

            andbdd == itebdd
        }
    }

    quickcheck! {
        fn bdd_ite_iff(c1: Cnf, c2: Cnf) -> TestResult {
            if c1.num_vars() == 0 || c1.num_vars() > 8 { return TestResult::discard() }
            if c1.clauses().len() > 12 { return TestResult::discard() }
            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(16);
            let cnf1 = mgr.from_cnf(&c1);
            let cnf2 = mgr.from_cnf(&c2);
            let iff1 = mgr.iff(cnf1, cnf2);

            let clause1 = mgr.and(cnf1, cnf2);
            let clause2 = mgr.and(cnf1.neg(), cnf2.neg());
            let and = mgr.or(clause1, clause2);

            if and != iff1 {
                println!("cnf1: {}", c1);
                println!("cnf2: {}", c2);
                println!("not equal: Bdd1: {}, Bdd2: {}", and.to_string_debug(), iff1.to_string_debug());
            }
            TestResult::from_bool(and == iff1)
        }
    }

    quickcheck! {
        fn compile_with_assignments(c1: Cnf) -> TestResult {
            if c1.num_vars() < 3 || c1.num_vars() > 8 { return TestResult::discard() }
            if c1.clauses().len() > 12 { return TestResult::discard() }
            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(c1.num_vars());
            let mut pm = PartialModel::from_litvec(&Vec::new(), c1.num_vars());
            pm.set(VarLabel::new(0), true);
            pm.set(VarLabel::new(1), true);
            let cnf1 = mgr.from_cnf_with_assignments(&c1, &pm);
            let mut cnf2 = mgr.from_cnf(&c1);
            cnf2 = mgr.condition(cnf2, VarLabel::new(0), true);
            cnf2 = mgr.condition(cnf2, VarLabel::new(1), true);
            assert_eq!(cnf1, cnf2);
            TestResult::from_bool(cnf1 == cnf2)
        }
    }

    quickcheck! {
        /// test that an BDD and CNF compute identical WMC
        fn wmc_eq(c1: Cnf) -> TestResult {
            // constrain the size
            if c1.num_vars() == 0 || c1.num_vars() > 8 { return TestResult::discard() }
            if c1.clauses().len() > 16 { return TestResult::discard() }

            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(c1.num_vars());
            let weight = create_semantic_hash_map::<{crate::BIG_PRIME}>(c1.num_vars());
            let cnf1 = mgr.from_cnf(&c1);
            let bddres = cnf1.wmc(mgr.get_order(), &weight);
            let cnfres = c1.wmc(&weight);
            TestResult::from_bool(bddres == cnfres)
        }
    }

    quickcheck! {
        /// test that an SDD and BDD both have the same semantic hash
        fn sdd_semantic_eq_bdd(c1: Cnf, vtree: VTree) -> bool {
            let bdd_mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(c1.num_vars());
            let sdd_mgr = super::CompressionSddManager::new(vtree);
            let map : WmcParams<rsdd::util::semirings::finitefield::FiniteField<{ crate::BIG_PRIME }>>= create_semantic_hash_map(c1.num_vars());
            let bdd = bdd_mgr.from_cnf(&c1);
            let sdd = sdd_mgr.from_cnf(&c1);
            bdd.semantic_hash(bdd_mgr.get_order(), &map) == sdd.semantic_hash(sdd_mgr.get_vtree_manager(), &map)
        }
    }

    quickcheck! {
        /// test that an SDD and BDD both have the same semantic hash with min-fill order
        fn sdd_semantic_eq_bdd_dtree(c1: Cnf) -> bool {
            let bdd_mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(c1.num_vars());
            let min_fill_order = c1.min_fill_order();
            let dtree = DTree::from_cnf(&c1, &min_fill_order);
            let vtree = VTree::from_dtree(&dtree).unwrap();

            let sdd_mgr = super::CompressionSddManager::new(vtree);
            let map : WmcParams<rsdd::util::semirings::finitefield::FiniteField<{ crate::BIG_PRIME }>>= create_semantic_hash_map(c1.num_vars());
            let bdd = bdd_mgr.from_cnf(&c1);
            let sdd = sdd_mgr.from_cnf(&c1);
            bdd.semantic_hash(bdd_mgr.get_order(), &map) == sdd.semantic_hash(sdd_mgr.get_vtree_manager(), &map)
        }
    }

    quickcheck! {
        fn wmc_bdd_dnnf_eq(c1: Cnf) -> TestResult {
            // constrain the size
            if c1.num_vars() == 0 || c1.num_vars() > 8 { return TestResult::discard() }
            if c1.clauses().len() > 16 { return TestResult::discard() }

            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(c1.num_vars());
            let weight_map : HashMap<VarLabel, (RealSemiring, RealSemiring)> = HashMap::from_iter(
                (0..16).map(|x| (VarLabel::new(x as u64), (RealSemiring(0.3), RealSemiring(0.7)))));
            let order = VarOrder::linear_order(c1.num_vars());
            let cnf1 = mgr.from_cnf(&c1);

            let mgr2 = DecisionNNFBuilder::new(order);
            let dnnf = mgr2.from_cnf_topdown(&c1);

            let bddwmc = super::repr::wmc::WmcParams::new_with_default(RealSemiring::zero(), RealSemiring::one(), weight_map);
            let bddres = cnf1.wmc(mgr.get_order(),  &bddwmc);
            let dnnfres = dnnf.wmc(mgr.get_order(), &bddwmc);
            let eps = f64::abs(bddres.0 - dnnfres.0) < 0.0001;
            if !eps {
              println!("error on input {}: bddres {}, cnfres {}\n topdown bdd: {}\nbottom-up bdd: {}",
                c1, bddres, dnnfres, dnnf.to_string_debug(), cnf1.to_string_debug());
            }
            TestResult::from_bool(eps)
        }
    }

    quickcheck! {
        /// test if the lru cache and the all cache give the same results
        fn bdd_lru(c1: Cnf) -> TestResult {
            let mgr1 = super::BddManager::<AllTable<BddPtr>>::new_default_order(16);
            let mgr2 = super::BddManager::<BddApplyTable<BddPtr>>::new_default_order_lru(16);

            let weight_map : HashMap<VarLabel, (RealSemiring, RealSemiring)> = HashMap::from_iter(
                (0..16).map(|x| (VarLabel::new(x as u64), (RealSemiring(0.3), RealSemiring(0.7)))));

            let bddwmc = super::repr::wmc::WmcParams::new_with_default(RealSemiring::zero(), RealSemiring::one(), weight_map);
            let cnf1 = mgr1.from_cnf(&c1);
            let cnf2 = mgr2.from_cnf(&c1);
            let wmc1 = cnf1.wmc(mgr1.get_order(), &bddwmc);
            let wmc2 = cnf2.wmc(mgr2.get_order(), &bddwmc);
            TestResult::from_bool(f64::abs(wmc1.0 - wmc2.0) < 0.00001)
        }
    }

    quickcheck! {
        fn marginal_map(c1: Cnf) -> TestResult {
            use rsdd::repr::model::PartialModel;
            // constrain the size
            if c1.num_vars() < 5 || c1.num_vars() > 8 { return TestResult::discard() }
            if c1.clauses().len() > 14 { return TestResult::discard() }

            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(c1.num_vars());
            let weight_map : HashMap<VarLabel, (RealSemiring, RealSemiring)> = HashMap::from_iter(
                (0..16).map(|x| (VarLabel::new(x as u64), (RealSemiring(0.3), RealSemiring(0.7)))));
            let cnf = mgr.from_cnf(&c1);
            let vars = vec![VarLabel::new(0), VarLabel::new(2), VarLabel::new(4)];
            if !c1.var_in_cnf(VarLabel::new(0))
               || !c1.var_in_cnf(VarLabel::new(2))
               || !c1.var_in_cnf(VarLabel::new(4)) {
                return TestResult::discard()
            }
            let wmc = WmcParams::new_with_default(RealSemiring::zero(), RealSemiring::one(), weight_map);

            let (marg_prob, marg_assgn) = cnf.marginal_map(&vars, mgr.num_vars(), &wmc);
            let (marg_prob_bb, marg_assgn_bb) = cnf.bb(&vars, mgr.num_vars(), &wmc);
            let assignments = vec![(true, true, true), (true, true, false), (true, false, true), (true, false, false),
                                   (false, true, true), (false, true, false), (false, false, true), (false, false, false)];

            let mut max : f64 = -10.0;
            let mut max_assgn : PartialModel = PartialModel::from_litvec(&[], c1.num_vars());
            for (v1, v2, v3) in assignments.iter() {
                let x = mgr.var(VarLabel::new(0), *v1);
                let y = mgr.var(VarLabel::new(2), *v2);
                let z = mgr.var(VarLabel::new(4), *v3);
                let mut conj = mgr.and(x, y);
                conj = mgr.and(conj, z);
                conj = mgr.and(conj, cnf);
                let poss_max = conj.wmc(mgr.get_order(), &wmc);
                if poss_max.0 > max {
                    max = poss_max.0;
                    max_assgn.set(VarLabel::new(0), *v1);
                    max_assgn.set(VarLabel::new(2), *v2);
                    max_assgn.set(VarLabel::new(4), *v3);
                }
            }

            // the below tests (specifically, the bool pm_check)
            // check that the partial models evaluate to the correct margmap.
            // these pms can be different b/c of symmetries/dead literals in the CNF.
            let mut pm_check = true;
            let extract = |ob : Option<bool>| -> bool {
                match ob {
                    Some(b) => b,
                    None => panic!("none encountered")
                }
            };
            let v : Vec<bool> = (0..3).map(|x| extract(marg_assgn.get(vars[x]))).collect();
            let w : Vec<bool> = (0..3).map(|x| extract(marg_assgn_bb.get(vars[x]))).collect();
            // if v != w {
            //     println!("{:?},{:?}",v,w);
            // }
            let v0 = mgr.var(vars[0], v[0]);
            let v1 = mgr.var(vars[1], v[1]);
            let v2 = mgr.var(vars[2], v[2]);
            let mut conj = mgr.and(v0, v1);
            conj = mgr.and(conj, v2);
            conj = mgr.and(conj, cnf);
            let poss_max = conj.wmc(mgr.get_order(), &wmc);
            if f64::abs(poss_max.0 - max) > 0.0001 {
                pm_check = false;
            }
            let w0 = mgr.var(vars[0], w[0]);
            let w1 = mgr.var(vars[1], w[1]);
            let w2 = mgr.var(vars[2], w[2]);
            let mut conj2 = mgr.and(w0, w1);
            conj2 = mgr.and(conj2, w2);
            mgr.and(conj2, cnf);
            let poss_max2 = conj.wmc(mgr.get_order(), &wmc);
            if f64::abs(poss_max2.0 - max) > 0.0001 {
                pm_check = false;
            }

            TestResult::from_bool(f64::abs(max - marg_prob) < 0.00001
                                  && f64::abs(marg_prob_bb.0 - marg_prob) < 0.00001
                                  && pm_check)
        }
    }

    quickcheck! {
        fn meu(c1: Cnf) -> TestResult {
            use rsdd::repr::model::PartialModel;
            let n = c1.num_vars();
            // constrain the size, make BDD
            if !(5..=8).contains(&n) { return TestResult::discard() }
            if c1.clauses().len() > 14 { return TestResult::discard() }
            let mgr = super::BddManager::<AllTable<BddPtr>>::new_default_order(n);
            let cnf = mgr.from_cnf(&c1);

            // randomizing the decisions
            let mut rng = rand::thread_rng();
            let decisions : Vec<VarLabel> = (0..3).map(|_| VarLabel::new(rng.gen_range(0..(n-2)) as u64)).collect();
            if decisions[0] == decisions[1] || decisions[1] == decisions[2] || decisions[0] == decisions[2] {
                return TestResult::discard()
            }
            if !c1.var_in_cnf(decisions[0])
               || !c1.var_in_cnf(decisions[1])
               || !c1.var_in_cnf(decisions[2]) {
                return TestResult::discard()
            }

            // weight function and weight map
            let probs : Vec<f64> = (0..n).map(|_| rng.gen_range(0.0..1.0)).collect();
            let weight_fn = |x : usize| -> (VarLabel, (ExpectedUtility, ExpectedUtility)) {
                let vx = VarLabel::new(x as u64);
                if vx == decisions[0] || vx == decisions[1] || vx == decisions[2] {
                    return (VarLabel::new(x as u64),
                    (ExpectedUtility::one(), ExpectedUtility::one()))
                }
                if x == n-1 || x == n-2 {
                    return (VarLabel::new(x as u64),
                    (ExpectedUtility::one(), ExpectedUtility(1.0, 10.0)))
                }
                let pr = probs[x];
                (vx, (ExpectedUtility(pr, 0.0), ExpectedUtility(1.0-pr, 0.0)))
            };
            let weight_map : HashMap<VarLabel, (ExpectedUtility, ExpectedUtility)> = HashMap::from_iter(
                (0..n).map(&weight_fn));

            // set up wmc, run meu
            let vars = decisions.clone();
            let wmc = WmcParams::new_with_default(ExpectedUtility::zero(), ExpectedUtility::one(), weight_map);

            let (meu , meu_assgn) = cnf.meu(&vars, mgr.num_vars(), &wmc);
            let (meu_bb, meu_assgn_bb) = cnf.bb(&vars, mgr.num_vars(), &wmc);

            // brute-force meu
            let assignments = vec![(true, true, true), (true, true, false), (true, false, true), (true, false, false),
                                   (false, true, true), (false, true, false), (false, false, true), (false, false, false)];
            let mut max : f64 = -10000.0;
            let mut max_assgn : PartialModel = PartialModel::from_litvec(&[], c1.num_vars());
            for (v1, v2, v3) in assignments.iter() {
                let x = mgr.var(decisions[0], *v1);
                let y = mgr.var(decisions[1], *v2);
                let z = mgr.var(decisions[2], *v3);
                let mut conj = mgr.and(x, y);
                conj = mgr.and(conj, z);
                conj = mgr.and(conj, cnf);
                let poss_max = conj.wmc(mgr.get_order(), &wmc);
                if poss_max.1 > max {
                    max = poss_max.1;
                    max_assgn.set(decisions[0], *v1);
                    max_assgn.set(decisions[1], *v2);
                    max_assgn.set(decisions[2], *v3);
                }
            }

            // and the actual checks.
            // these checks test that the meus coincide.
            let pr_check1 = f64::abs(meu.1 - meu_bb.1) < 0.00001;
            let pr_check2 = f64::abs(max - meu.1)< 0.00001;

            // the below tests (specifically, the bool pm_check)
            // check that the partial models evaluate to the correct meu.
            // these pms can be different b/c of symmetries/dead literals in the CNF.
            let mut pm_check = true;
            let extract = |ob : Option<bool>| -> bool {
                match ob {
                    Some(b) => b,
                    None => panic!("none encountered")
                }
            };
            let v : Vec<bool> = (0..3).map(|x| extract(meu_assgn.get(decisions[x]))).collect();
            let w : Vec<bool> = (0..3).map(|x| extract(meu_assgn_bb.get(decisions[x]))).collect();
            // if v != w {
            //     println!("{:?},{:?}",v,w);
            // }
            let v0 = mgr.var(decisions[0], v[0]);
            let v1 = mgr.var(decisions[1], v[1]);
            let v2 = mgr.var(decisions[2], v[2]);
            let mut conj = mgr.and(v0, v1);
            conj = mgr.and(conj, v2);
            conj = mgr.and(conj, cnf);
            let poss_max = conj.wmc(mgr.get_order(), &wmc);
            if f64::abs(poss_max.1 - max) > 0.0001 {
                pm_check = false;
            }
            let w0 = mgr.var(decisions[0], w[0]);
            let w1 = mgr.var(decisions[1], w[1]);
            let w2 = mgr.var(decisions[2], w[2]);
            let mut conj2 = mgr.and(w0, w1);
            conj2 = mgr.and(conj2, w2);
            mgr.and(conj2, cnf);
            let poss_max2 = conj.wmc(mgr.get_order(), &wmc);
            if f64::abs(poss_max2.1 - max) > 0.0001 {
                pm_check = false;
            }

            TestResult::from_bool(pr_check1 && pr_check2 && pm_check)
        }
    }
}

#[cfg(test)]
mod test_sdd_manager {
    use crate::builder::bdd_builder::BddManager;
    use crate::repr::cnf::Cnf;
    use crate::repr::var_label::{Literal, VarLabel};
    use crate::rsdd::builder::BottomUpBuilder;
    use quickcheck::{Arbitrary, TestResult};
    use rand::rngs::SmallRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    use rsdd::builder::cache::all_app::AllTable;
    use rsdd::builder::sdd_builder::{CompressionSddManager, SddBuilder, SemanticSddManager};
    use rsdd::repr::ddnnf::{create_semantic_hash_map, DDNNFPtr};
    use rsdd::repr::dtree::DTree;
    use rsdd::repr::robdd::BddPtr;
    use rsdd::repr::sdd::SddPtr;
    use rsdd::repr::var_order::VarOrder;
    use rsdd::repr::vtree::VTree;
    use rsdd::repr::wmc::WmcParams;
    use rsdd::util::semirings::finitefield::FiniteField;
    use std::collections::HashMap;

    quickcheck! {
        fn test_cond_and(c: Cnf) -> bool {
            let order : Vec<VarLabel> = (0..16).map(VarLabel::new).collect();
            let mgr = super::CompressionSddManager::new(VTree::even_split(&order, 4));
            let cnf = mgr.from_cnf(&c);
            let v1 = VarLabel::new(0);
            let bdd1 = mgr.exists(cnf, v1);

            let bdd2 = mgr.condition(cnf, v1, true);
            let bdd3 = mgr.condition(cnf, v1, false);
            let bdd4 = mgr.or(bdd2, bdd3);
            bdd4 == bdd1
        }
    }

    quickcheck! {
        fn ite_iff_rightlinear(c1: Cnf, c2: Cnf) -> bool {
            // println!("testing with cnf {:?}, {:?}", c1, c2);
            let order : Vec<VarLabel> = (0..16).map(VarLabel::new).collect();
            // let vtree = VTree::even_split(&order, 4);
            let vtree = VTree::right_linear(&order);
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf1 = mgr.from_cnf(&c1);
            let cnf2 = mgr.from_cnf(&c2);
            let iff1 = mgr.iff(cnf1, cnf2);

            let clause1 = mgr.and(cnf1, cnf2);
            let clause2 = mgr.and(cnf1.neg(), cnf2.neg());
            let and = mgr.or(clause1, clause2);

            if and != iff1 {
                println!("Not equal:\n{}\n{}", mgr.print_sdd(and), mgr.print_sdd(iff1));
            }

            and == iff1
        }
    }

    quickcheck! {
        fn ite_iff_split(c1: Cnf, c2: Cnf) -> bool {
            let order : Vec<VarLabel> = (0..16).map(VarLabel::new).collect();
            let vtree = VTree::even_split(&order, 4);
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf1 = mgr.from_cnf(&c1);
            let cnf2 = mgr.from_cnf(&c2);
            let iff1 = mgr.iff(cnf1, cnf2);

            let clause1 = mgr.and(cnf1, cnf2);
            let clause2 = mgr.and(cnf1.neg(), cnf2.neg());
            let and = mgr.or(clause1, clause2);

            if and != iff1 {
                println!("Not equal:\n{}\n{}", mgr.print_sdd(and), mgr.print_sdd(iff1));
            }

            and == iff1
        }
    }

    quickcheck! {
        /// test that the same CNF compiled by both an SDD and BDD have the same weighted model count
        /// with an even_split ordering
        fn sdd_wmc_eq_even_split(clauses: Vec<Vec<Literal>>) -> TestResult {
            let cnf = Cnf::new(clauses);
            if cnf.num_vars() < 8 || cnf.num_vars() > 16 { return TestResult::discard() }
            if cnf.clauses().len() > 16 { return TestResult::discard() }

           let weight_map = create_semantic_hash_map::< {crate::BIG_PRIME} >(cnf.num_vars());
           let order : Vec<VarLabel> = (0..cnf.num_vars()).map(|x| VarLabel::new(x as u64)).collect();
           let mgr = super::CompressionSddManager::new(VTree::even_split(&order, 3));
           let cnf_sdd = mgr.from_cnf(&cnf);
           let sdd_res = cnf_sdd.semantic_hash(mgr.get_vtree_manager(), &weight_map);


            let bddmgr = BddManager::<AllTable<BddPtr>>::new_default_order(cnf.num_vars());
            let cnf_bdd = bddmgr.from_cnf(&cnf);
            let bdd_res = cnf_bdd.semantic_hash(bddmgr.get_order(), &weight_map);
            assert_eq!(bdd_res, sdd_res);
            TestResult::passed()
        }
    }

    quickcheck! {
        /// test that the same CNF compiled by both an SDD and BDD have the same weighted model count
        /// with a dtree ordering
        fn sdd_wmc_eq(clauses: Vec<Vec<Literal>>) -> TestResult {
            let cnf = Cnf::new(clauses);
            if cnf.num_vars() < 8 || cnf.num_vars() > 16 { return TestResult::discard() }
            if cnf.clauses().len() > 16 { return TestResult::discard() }

            let dtree = DTree::from_cnf(&cnf, &VarOrder::linear_order(cnf.num_vars()));
            let vtree = VTree::from_dtree(&dtree).unwrap();

            let weight_map = create_semantic_hash_map::< {crate::BIG_PRIME} >(cnf.num_vars());
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf_sdd = mgr.from_cnf(&cnf);
            let sdd_res = cnf_sdd.semantic_hash(mgr.get_vtree_manager(), &weight_map);


            let bddmgr = BddManager::<AllTable<BddPtr>>::new_default_order(cnf.num_vars());
            let cnf_bdd = bddmgr.from_cnf(&cnf);
            let bdd_res = cnf_bdd.semantic_hash(bddmgr.get_order(), &weight_map);
            assert_eq!(bdd_res, sdd_res);
            TestResult::passed()
        }
    }

    // why does this exist?
    // well, I wasn't able to figure out how to generate a random permutation of vectors from 0..16 with quickcheck
    #[derive(Clone, Debug)]
    struct SixteenVarLabels {
        order: Vec<VarLabel>,
    }

    impl Arbitrary for SixteenVarLabels {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = SmallRng::seed_from_u64(u64::arbitrary(g));
            let mut order: Vec<VarLabel> = (0..16).map(VarLabel::new).collect();
            order.shuffle(&mut rng);
            SixteenVarLabels { order }
        }
    }

    quickcheck! {
        fn sdd_left_linear_predicate(s: SixteenVarLabels) -> bool {
            let vtree = VTree::left_linear(&s.order);
            vtree.is_left_linear()
        }
    }

    quickcheck! {
        fn sdd_right_linear_predicate(s: SixteenVarLabels) -> bool {
            let vtree = VTree::right_linear(&s.order);
            vtree.is_right_linear()
        }
    }

    quickcheck! {
        fn sdd_compressed_right_linear(c: Cnf) -> bool {
            let order : Vec<VarLabel> = (0..16).map(VarLabel::new).collect();
            let vtree = VTree::right_linear(&order);
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf = mgr.from_cnf(&c);
            cnf.is_compressed()
        }
    }

    quickcheck! {
        fn sdd_trimmed_right_linear(c: Cnf) -> bool {
            let order : Vec<VarLabel> = (0..16).map(VarLabel::new).collect();
            let vtree = VTree::right_linear(&order);
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf = mgr.from_cnf(&c);

            cnf.is_trimmed()
        }
    }

    quickcheck! {
        fn sdd_compressed_arbitrary_vtree(c: Cnf, vtree: VTree) -> bool {
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf = mgr.from_cnf(&c);
            cnf.is_compressed()
        }
    }

    quickcheck! {
        fn sdd_trimmed_arbitrary_vtree(c: Cnf, vtree: VTree) -> bool {
            let mgr = super::CompressionSddManager::new(vtree);
            let cnf = mgr.from_cnf(&c);
            cnf.is_trimmed()
        }
    }

    quickcheck! {
        fn prob_equiv_trivial(c: Cnf, vtree:VTree) -> bool {
            let mgr1 = CompressionSddManager::new(vtree.clone());
            let c1 = mgr1.from_cnf(&c);

            // in this test, compression is still enabled; c2 should be identical to c1
            let mgr2 = SemanticSddManager::<{ crate::BIG_PRIME }>::new(vtree);
            let c2 = mgr2.from_cnf(&c);

            let map : WmcParams<FiniteField<{ crate::BIG_PRIME }>> = create_semantic_hash_map(mgr1.num_vars());

            let h1 = c1.semantic_hash(mgr1.get_vtree_manager(), &map);
            let h2 = c2.semantic_hash(mgr2.get_vtree_manager(), &map);

            h1 == h2
        }
    }

    quickcheck! {
        fn prob_equiv_sdd_identity_uncompressed_depr(c: Cnf, vtree:VTree) -> TestResult {
            let compr_mgr = super::CompressionSddManager::new(vtree.clone());
            let compr_cnf = compr_mgr.from_cnf(&c);

            let mut uncompr_mgr = super::CompressionSddManager::new(vtree);
            uncompr_mgr.set_compression(false);
            let uncompr_cnf = uncompr_mgr.from_cnf(&c);

            let map : WmcParams<FiniteField<{ crate::BIG_PRIME }>> = create_semantic_hash_map(compr_mgr.num_vars());

            let compr_h = compr_cnf.semantic_hash(compr_mgr.get_vtree_manager(), &map);
            let uncompr_h = uncompr_cnf.semantic_hash(uncompr_mgr.get_vtree_manager(), &map);

            if compr_h != uncompr_h {
                println!("not equal! hashes: compr: {compr_h}, uncompr: {uncompr_h}");
                println!("map: {:?}", map);
                println!("compr sdd: {}", compr_mgr.print_sdd(compr_cnf));
                println!("uncompr sdd: {}", uncompr_mgr.print_sdd(uncompr_cnf));
                TestResult::from_bool(false)
            } else {
                TestResult::from_bool(true)
            }
        }
    }

    quickcheck! {
        fn prob_equiv_sdd_identity_uncompressed(c: Cnf, vtree:VTree) -> TestResult {
            let compr_mgr = CompressionSddManager::new(vtree.clone());
            let compr_cnf = compr_mgr.from_cnf(&c);

            let uncompr_mgr = SemanticSddManager::<{ crate::BIG_PRIME }>::new(vtree);
            let uncompr_cnf = uncompr_mgr.from_cnf(&c);

            if !uncompr_mgr.sdd_eq(compr_cnf, uncompr_cnf) {
                println!("not equal!");
                println!("compr sdd: {}", compr_mgr.print_sdd(compr_cnf));
                println!("uncompr sdd: {}", uncompr_mgr.print_sdd(uncompr_cnf));
                TestResult::from_bool(false)
            } else {
                TestResult::from_bool(true)
            }
        }
    }

    quickcheck! {
        fn prob_equiv_sdd_inequality(c1: Cnf, c2: Cnf, vtree:VTree) -> TestResult {
            let mut mgr = SemanticSddManager::<{ crate::BIG_PRIME }>::new(vtree);
            mgr.set_compression(true); // necessary to make sure we don't generate two uncompressed SDDs that canonicalize to the same SDD
            let cnf_1 = mgr.from_cnf(&c1);
            let cnf_2 = mgr.from_cnf(&c2);

            if cnf_1 == cnf_2 {
                return TestResult::discard();
            }

            if mgr.sdd_eq(cnf_1, cnf_2) {
                println!("collision!");
                println!("sdd 1: {}", mgr.print_sdd(cnf_1));
                println!("sdd 2: {}", mgr.print_sdd(cnf_2));
                TestResult::from_bool(false)
            } else {
                TestResult::from_bool(true)
            }
        }
    }

    quickcheck! {
        fn prob_equiv_sdd_eq_vs_prob_eq(c1: Cnf, c2: Cnf, vtree:VTree) -> TestResult {
            let mut mgr = SemanticSddManager::<{ crate::BIG_PRIME }>::new(vtree);
            mgr.set_compression(true); // necessary to make sure we don't generate two uncompressed SDDs that canonicalize to the same SDD
            let cnf_1 = mgr.from_cnf(&c1);
            let cnf_2 = mgr.from_cnf(&c2);

            let h_eq = mgr.sdd_eq(cnf_1, cnf_2);

            if h_eq != (cnf_1 == cnf_2) {
                println!("disagreement!");
                println!("ptr eq: {}, mgr_eq: {}", cnf_1 == cnf_2, h_eq);
                println!("sdd 1: {}", mgr.print_sdd(cnf_1));
                println!("sdd 2: {}", mgr.print_sdd(cnf_2));
                TestResult::from_bool(false)
            } else {
                TestResult::from_bool(true)
            }
        }
    }

    quickcheck! {
        /// verify that every node in the SDD compression canonicalizer has a unique semantic hash, using CompressionCanonicalizer
        fn qc_sdd_canonicity(c1: Cnf, vtree:VTree) -> TestResult {
            let mgr = super::CompressionSddManager::new(vtree);
            let _ = mgr.from_cnf(&c1);

            let map : WmcParams<FiniteField<{ crate::BIG_PRIME }>>= create_semantic_hash_map(mgr.num_vars());
            let mut seen_hashes : HashMap<u128, SddPtr> = HashMap::new();
            for sdd in mgr.node_iter() {
                let hash = sdd.semantic_hash(mgr.get_vtree_manager(), &map);
                if seen_hashes.contains_key(&hash.value()) {
                    let c = seen_hashes.get(&hash.value()).unwrap();
                    println!("cnf: {}", c1);
                    println!("probmap: {:?}", map);
                    println!("collision found for hash value {}", hash);
                    println!("sdd a: {}\n", mgr.print_sdd(sdd));
                    println!("sdd b: {}\n", mgr.print_sdd(*c));
                    return TestResult::from_bool(false);
                }
                seen_hashes.insert(hash.value(), sdd);
            }
            TestResult::from_bool(true)
        }
    }

    quickcheck! {
        /// verify that every node in the SDD with the semantic canonicalizer a unique semantic hash w.r.t negations
        /// using SemanticCanonicalizer
        fn qc_semantic_sdd_canonicity(c1: Cnf, vtree:VTree) -> TestResult {
            let mgr = SemanticSddManager::< {crate::BIG_PRIME} >::new(vtree);
            let _ = mgr.from_cnf(&c1);

            let map : WmcParams<FiniteField<{ crate::BIG_PRIME }>>= create_semantic_hash_map(mgr.num_vars());
            let mut seen_hashes : HashMap<u128, SddPtr> = HashMap::new();
            for sdd in mgr.node_iter() {
                let hash = sdd.semantic_hash(mgr.get_vtree_manager(), &map);

                // see the hash itself
                if seen_hashes.contains_key(&hash.value()) {
                    let c = seen_hashes.get(&hash.value()).unwrap();
                    println!("cnf: {}", c1);
                    println!("probmap: {:?}", map);
                    println!("collision found for hash value {}", hash);
                    println!("sdd a: {}\n", mgr.print_sdd(sdd));
                    println!("sdd b: {}\n", mgr.print_sdd(*c));
                    return TestResult::from_bool(false);
                }

                // see the hash's negation
                if seen_hashes.contains_key(&hash.negate().value()) {
                    let c = seen_hashes.get(&hash.negate().value()).unwrap();
                    println!("cnf: {}", c1);
                    println!("probmap: {:?}", map);
                    println!("collision found for negated hash value {}", hash.negate());
                    println!("sdd a: {}\n", mgr.print_sdd(sdd));
                    println!("sdd b: {}\n", mgr.print_sdd(*c));
                    return TestResult::from_bool(false);
                }
                seen_hashes.insert(hash.value(), sdd);
            }
            TestResult::from_bool(true)
        }
    }

    quickcheck! {
        fn vtree_validity_arbitrary(vtree: VTree) -> bool {
            VTree::is_valid_vtree(&vtree)
        }
    }

    quickcheck! {
        fn vtree_validity_from_dtree(cnf: Cnf) -> bool {
            let dtree = rsdd::repr::dtree::DTree::from_cnf(&cnf, &cnf.min_fill_order());
            let vtree = VTree::from_dtree(&dtree).unwrap();
            VTree::is_valid_vtree(&vtree)
        }
    }

    quickcheck! {
        /// verify that the semantic hash of an SDDPtr + its compl is always equal to 1
        fn semantic_reg_plus_compl_eq_one(c1: Cnf, vtree:VTree) -> bool {
            let mgr = SemanticSddManager::<{ crate::BIG_PRIME }>::new(vtree);
            let map : WmcParams<FiniteField<{ crate::BIG_PRIME }>>= create_semantic_hash_map(mgr.num_vars());

            let sdd = mgr.from_cnf(&c1);
            let compl = sdd.neg();

            let sdd_hash = sdd.semantic_hash(mgr.get_vtree_manager(), &map);
            let compl_hash = compl.semantic_hash(mgr.get_vtree_manager(), &map);

            let sum = (sdd_hash + compl_hash).value();

            if sum != 1 {
                println!("hashes do not sum to one; Reg: {}, Compl: {}", sdd_hash, compl_hash);
            }

            sum == 1
        }
    }
}
