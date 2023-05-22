use super::cache::sdd_apply_cache::{SddApply, SddApplyCompression};
use crate::backing_store::bump_table::BackedRobinhoodTable;
use crate::backing_store::{DefaultUniqueTableHasher, UniqueTable, UniqueTableHasher};
use crate::repr::sdd::binary_sdd::BinarySDD;
use crate::repr::sdd::sdd_or::SddOr;
use crate::repr::sdd::SddPtr;
use crate::repr::vtree::VTreeManager;

pub trait SddCanonicalizationScheme<'a> {
    type ApplyCacheMethod: SddApply<'a>;
    type BddHasher: UniqueTableHasher<BinarySDD<'a>>;
    type SddOrHasher: UniqueTableHasher<SddOr<'a>>;

    fn new(vtree: &VTreeManager) -> Self;
    fn set_compress(&mut self, b: bool);
    fn should_compress(&self) -> bool;

    /// this is mutable because we may update an internal cache
    fn sdd_eq(&'a self, a: SddPtr<'a>, b: SddPtr<'a>) -> bool;
    fn app_cache(&self) -> &mut Self::ApplyCacheMethod;

    // BackedRobinhoodTable-related methods
    fn bdd_tbl(&self) -> &BackedRobinhoodTable<BinarySDD>;
    fn sdd_tbl(&self) -> &BackedRobinhoodTable<SddOr>;
    fn node_iter(&self) -> Vec<SddPtr>;

    fn bdd_get_or_insert(&'a mut self, item: BinarySDD<'a>) -> SddPtr<'a>;
    fn sdd_get_or_insert(&'a mut self, item: SddOr<'a>) -> SddPtr<'a>;

    // debugging util
    fn on_sdd_print_dump_state(&'a self, ptr: SddPtr<'a>);
}

pub struct CompressionCanonicalizer<'a> {
    use_compression: bool,
    app_cache: SddApplyCompression<'a>,
    bdd_tbl: BackedRobinhoodTable<'a, BinarySDD<'a>>,
    sdd_tbl: BackedRobinhoodTable<'a, SddOr<'a>>,
    hasher: DefaultUniqueTableHasher,
}

impl<'a> SddCanonicalizationScheme<'a> for CompressionCanonicalizer<'a> {
    type ApplyCacheMethod = SddApplyCompression<'a>;
    type BddHasher = DefaultUniqueTableHasher;
    type SddOrHasher = DefaultUniqueTableHasher;

    fn new(_vtree: &VTreeManager) -> Self {
        CompressionCanonicalizer {
            use_compression: true,
            app_cache: SddApplyCompression::new(),
            bdd_tbl: BackedRobinhoodTable::new(),
            sdd_tbl: BackedRobinhoodTable::new(),
            hasher: DefaultUniqueTableHasher::default(),
        }
    }

    fn sdd_eq(&self, a: SddPtr, b: SddPtr) -> bool {
        a == b
    }

    fn set_compress(&mut self, b: bool) {
        self.use_compression = b
    }

    fn should_compress(&self) -> bool {
        self.use_compression
    }

    fn app_cache(&self) -> &mut Self::ApplyCacheMethod {
        todo!("make this typecheck with interior mutability");
        &mut self.app_cache
    }

    fn bdd_tbl(&self) -> &BackedRobinhoodTable<BinarySDD> {
        &self.bdd_tbl
    }

    fn sdd_tbl(&self) -> &BackedRobinhoodTable<SddOr> {
        &self.sdd_tbl
    }

    fn node_iter(&self) -> Vec<SddPtr> {
        let bdds = self.bdd_tbl().iter().map(|x| SddPtr::bdd(x));
        let sdds = self.sdd_tbl().iter().map(|x| SddPtr::Reg(x));
        bdds.chain(sdds).collect()
    }

    fn bdd_get_or_insert(&'a mut self, item: BinarySDD<'a>) -> SddPtr<'a> {
        SddPtr::BDD(self.bdd_tbl.get_or_insert(item, &self.hasher))
    }

    fn sdd_get_or_insert(&'a mut self, item: SddOr<'a>) -> SddPtr<'a> {
        SddPtr::or(self.sdd_tbl.get_or_insert(item, &self.hasher))
    }

    fn on_sdd_print_dump_state(&self, _ptr: SddPtr) {}
}

// pub struct SemanticUniqueTableHasher<const P: u128> {
//     map: WmcParams<FiniteField<P>>,
//     vtree: VTreeManager,
// }

// impl<const P: u128> SemanticUniqueTableHasher<P> {
//     pub fn new(vtree: VTreeManager, map: WmcParams<FiniteField<P>>) -> Self {
//         Self { vtree, map }
//     }
// }

// impl<'a, const P: u128> UniqueTableHasher<BinarySDD<'a>> for SemanticUniqueTableHasher<P> {
//     fn u64hash(&self, elem: &BinarySDD) -> u64 {
//         let mut hasher = FxHasher::default();
//         elem.semantic_hash(&self.vtree, &self.map)
//             .value()
//             .hash(&mut hasher);
//         hasher.finish()
//     }
// }

// impl<'a, const P: u128> UniqueTableHasher<SddOr<'a>> for SemanticUniqueTableHasher<P> {
//     fn u64hash(&self, elem: &SddOr) -> u64 {
//         let mut hasher = FxHasher::default();
//         elem.semantic_hash(&self.vtree, &self.map)
//             .value()
//             .hash(&mut hasher);
//         hasher.finish()
//     }
// }

// pub struct SemanticCanonicalizer<'a, const P: u128> {
//     map: WmcParams<FiniteField<P>>,
//     app_cache: SddApplySemantic<'a, P>,
//     use_compression: bool,
//     vtree: VTreeManager,
//     bdd_tbl: BackedRobinhoodTable<'a, BinarySDD<'a>>,
//     sdd_tbl: BackedRobinhoodTable<'a, SddOr<'a>>,
//     hasher: SemanticUniqueTableHasher<P>,
// }

// impl<'a, const P: u128> SemanticCanonicalizer<'a, P> {
//     fn get_shared_sdd_ptr(
//         &'a self,
//         semantic_hash: FiniteField<P>,
//         hash: u64,
//     ) -> Option<SddPtr<'a>> {
//         match semantic_hash.value() {
//             0 => Some(SddPtr::PtrFalse),
//             1 => Some(SddPtr::PtrTrue),
//             _ => {
//                 if let Some(sdd) = <BackedRobinhoodTable<BinarySDD> as UniqueTable<
//                     BinarySDD,
//                     SemanticUniqueTableHasher<P>,
//                 >>::get_by_hash(&self.bdd_tbl, hash)
//                 {
//                     return Some(SddPtr::BDD(sdd));
//                 }
//                 if let Some(sdd) = <BackedRobinhoodTable<SddOr> as UniqueTable<
//                     SddOr,
//                     SemanticUniqueTableHasher<P>,
//                 >>::get_by_hash(&self.sdd_tbl, hash)
//                 {
//                     return Some(SddPtr::or(sdd));
//                 }
//                 None
//             }
//         }
//     }

//     fn check_cached_hash_and_neg(&'a self, semantic_hash: FiniteField<P>) -> Option<SddPtr<'a>> {
//         // check regular hash
//         let mut hasher = FxHasher::default();
//         semantic_hash.value().hash(&mut hasher);
//         let hash = hasher.finish();
//         if let Some(sdd) = self.get_shared_sdd_ptr(semantic_hash, hash) {
//             return Some(sdd);
//         }

//         // check negated hash
//         let semantic_hash = semantic_hash.negate();
//         let mut hasher = FxHasher::default();
//         semantic_hash.value().hash(&mut hasher);
//         let hash = hasher.finish();
//         if let Some(sdd) = self.get_shared_sdd_ptr(semantic_hash, hash) {
//             return Some(sdd.neg());
//         }
//         None
//     }
// }
// impl<'a, const P: u128> SddCanonicalizationScheme<'a> for SemanticCanonicalizer<'a, P> {
//     type ApplyCacheMethod = SddApplySemantic<'a, P>;
//     type BddHasher = SemanticUniqueTableHasher<P>;
//     type SddOrHasher = SemanticUniqueTableHasher<P>;

//     fn new(vtree: &VTreeManager) -> Self {
//         let map = create_semantic_hash_map(vtree.vtree_root().num_vars());
//         let app_cache = SddApplySemantic::new(map.clone(), vtree.clone());
//         SemanticCanonicalizer {
//             app_cache,
//             use_compression: false,
//             vtree: vtree.clone(),
//             bdd_tbl: BackedRobinhoodTable::new(),
//             sdd_tbl: BackedRobinhoodTable::new(),
//             hasher: SemanticUniqueTableHasher::new(vtree.clone(), map.clone()),
//             map,
//         }
//     }

//     fn sdd_eq(&self, a: SddPtr, b: SddPtr) -> bool {
//         let h1 = a.cached_semantic_hash(&self.vtree, &self.map);
//         let h2 = b.cached_semantic_hash(&self.vtree, &self.map);
//         h1 == h2
//     }

//     fn set_compress(&mut self, b: bool) {
//         self.use_compression = b
//     }

//     fn should_compress(&self) -> bool {
//         self.use_compression
//     }

//     fn app_cache(&self) -> &mut Self::ApplyCacheMethod {
//         todo!("make this typecheck with interior mutability");
//         &mut self.app_cache
//     }

//     fn bdd_tbl(&self) -> &BackedRobinhoodTable<BinarySDD> {
//         &self.bdd_tbl
//     }

//     fn sdd_tbl(&self) -> &BackedRobinhoodTable<SddOr> {
//         &self.sdd_tbl
//     }

//     fn bdd_get_or_insert(&'a mut self, item: BinarySDD<'a>) -> SddPtr<'a> {
//         todo!(); // make this typecheck with interior mutability
//         let semantic_hash = item.semantic_hash(&self.vtree, &self.map);
//         if let Some(sdd) = self.check_cached_hash_and_neg(semantic_hash) {
//             return sdd;
//         }

//         SddPtr::BDD(self.bdd_tbl.get_or_insert(item, &self.hasher))
//     }

//     fn sdd_get_or_insert(&'a mut self, item: SddOr<'a>) -> SddPtr<'a> {
//         todo!(); // make this typecheck with interior mutability
//         let semantic_hash = item.semantic_hash(&self.vtree, &self.map);
//         if let Some(sdd) = self.check_cached_hash_and_neg(semantic_hash) {
//             return sdd;
//         }

//         SddPtr::or(self.sdd_tbl.get_or_insert(item, &self.hasher))
//     }

//     fn on_sdd_print_dump_state(&self, ptr: SddPtr) {
//         println!("h: {}", ptr.semantic_hash(&self.vtree, &self.map));
//     }
// }
