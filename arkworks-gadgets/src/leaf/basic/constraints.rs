use super::Private;
use crate::Vec;
use ark_crypto_primitives::{crh::CRHGadget, CRH};
use ark_ff::fields::PrimeField;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{Namespace, SynthesisError};
use ark_std::marker::PhantomData;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct PrivateVar<F: PrimeField> {
	r: FpVar<F>,
	nullifier: FpVar<F>,
}

#[derive(Clone, Default)]
pub struct PublicVar<F: PrimeField> {
	f: PhantomData<F>,
}

impl<F: PrimeField> PrivateVar<F> {
	pub fn new(r: FpVar<F>, nullifier: FpVar<F>) -> Self {
		Self { r, nullifier }
	}
}

pub struct BasicLeafGadget<F: PrimeField, H: CRH, HG: CRHGadget<H, F>> {
	field: PhantomData<F>,
	hasher: PhantomData<H>,
	hasher_gadget: PhantomData<HG>,
}

impl<F: PrimeField, H: CRH, HG: CRHGadget<H, F>> BasicLeafGadget<F, H, HG> {
	pub fn create_leaf(
		private: &PrivateVar<F>,
		h: &HG::ParametersVar,
	) -> Result<HG::OutputVar, SynthesisError> {
		let mut bytes = Vec::new();
		bytes.extend(private.r.to_bytes()?);
		bytes.extend(private.nullifier.to_bytes()?);
		HG::evaluate(h, &bytes)
	}

	pub fn create_nullifier(
		private: &PrivateVar<F>,
		h: &HG::ParametersVar,
	) -> Result<HG::OutputVar, SynthesisError> {
		let mut bytes = Vec::new();
		bytes.extend(private.nullifier.to_bytes()?);
		bytes.extend(private.nullifier.to_bytes()?);
		HG::evaluate(h, &bytes)
	}
}

impl<F: PrimeField> AllocVar<Private<F>, F> for PrivateVar<F> {
	fn new_variable<T: Borrow<Private<F>>>(
		into_ns: impl Into<Namespace<F>>,
		f: impl FnOnce() -> Result<T, SynthesisError>,
		mode: AllocationMode,
	) -> Result<Self, SynthesisError> {
		let secrets = f()?.borrow().clone();
		let ns = into_ns.into();
		let cs = ns.cs();

		let r = secrets.r;
		let nullifier = secrets.nullifier;
		let r_var = FpVar::new_variable(cs.clone(), || Ok(r), mode)?;
		let nullifier_var = FpVar::new_variable(cs, || Ok(nullifier), mode)?;
		Ok(PrivateVar::new(r_var, nullifier_var))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{
		leaf::basic::BasicLeaf,
		poseidon::{
			constraints::{CRHGadget, PoseidonParametersVar},
			CRH,
		},
	};
	use ark_bls12_381::Fq;
	use ark_relations::r1cs::ConstraintSystem;
	use ark_std::test_rng;
	use arkworks_utils::utils::common::setup_params_x5_3;

	type PoseidonCRH3 = CRH<Fq>;
	type PoseidonCRH3Gadget = CRHGadget<Fq>;

	type Leaf = BasicLeaf<Fq, PoseidonCRH3>;
	type LeafGadget = BasicLeafGadget<Fq, PoseidonCRH3, PoseidonCRH3Gadget>;
	#[test]
	fn should_crate_basic_leaf_constraints() {
		let rng = &mut test_rng();
		let curve = arkworks_utils::utils::common::Curve::Bls381;

		let cs = ConstraintSystem::<Fq>::new_ref();

		// Native version
		let secrets = Private::generate(rng);
		let params = setup_params_x5_3(curve);
		let leaf = Leaf::create_leaf(&secrets, &params).unwrap();

		// Constraints version
		let secrets_var = PrivateVar::new_witness(cs.clone(), || Ok(&secrets)).unwrap();
		let params_var =
			PoseidonParametersVar::new_variable(cs, || Ok(&params), AllocationMode::Constant)
				.unwrap();

		let leaf_var = LeafGadget::create_leaf(&secrets_var, &params_var).unwrap();

		// Check equality
		let leaf_new_var = FpVar::<Fq>::new_witness(leaf_var.cs(), || Ok(leaf)).unwrap();
		let res = leaf_var.is_eq(&leaf_new_var).unwrap();
		assert!(res.value().unwrap());
		assert!(res.cs().is_satisfied().unwrap());
	}
}
