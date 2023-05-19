#[allow(unused_imports)]

use std::marker::PhantomData;
use ark_ff::{
    fields::Fp64,
    fields::{MontBackend, MontConfig},
    Field, One, PrimeField,Zero
};

use ark_poly::polynomial::multivariate::{self,SparsePolynomial, SparseTerm, Term};
use ark_poly::polynomial::{univariate,DenseMVPolynomial};
use ark_poly::Polynomial;
use ark_std::cfg_into_iter;


// https://cronokirby.com/notes/2022/09/the-goldilocks-field/
#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "7"]
pub struct FrConfig;
#[allow(non_camel_case_types)]
type GL_Field = Fp64<MontBackend<FrConfig, 1>>;


fn n_to_vec(i: usize, n: usize) -> Vec<GL_Field> {
	format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()
}

pub trait SumCheckPolynomial<F: Field> {
    fn evaluate(&self, point: &[F]) -> Option<F>;
    fn fix_variables(&self, partial_point: &[F]) -> Self;
    fn to_univariate(&self) -> univariate::SparsePolynomial<F>;
    fn num_vars(&self) -> usize;
    fn to_evaluations(&self) -> Vec<F>;
}

impl<F: Field> SumCheckPolynomial<F> for multivariate::SparsePolynomial<F, SparseTerm> {
    fn evaluate(&self, point: &[F]) -> Option<F> {
        Some(Polynomial::evaluate(self, &point.to_owned()))
    }

    fn fix_variables(&self, partial_point: &[F]) -> Self {
        let mut res = Self::zero();
        let num_vars = DenseMVPolynomial::num_vars(self);
        let mut full_point = partial_point.to_vec();
        full_point.append(&mut vec![F::one(); num_vars - partial_point.len()]);

        for (coeff, term) in &self.terms {
            let mut eval = term.evaluate(&full_point);
            eval *= coeff;
            let new_term = SparseTerm::new(
                term.iter()
                    .filter(|(var, _)| *var >= partial_point.len())
                    .map(|(var, power)| (var - partial_point.len(), *power))
                    .collect(),
            );
            let poly = multivariate::SparsePolynomial {
                num_vars: num_vars - partial_point.len(),
                terms: vec![(eval, new_term)],
            };

            res += &poly;
        }

        res
    }

    fn to_univariate(&self) -> univariate::SparsePolynomial<F> {
        let mut res = univariate::SparsePolynomial::zero();

        res
    }

    fn num_vars(&self) -> usize {
        DenseMVPolynomial::num_vars(self)
    }
    fn to_evaluations(&self) -> Vec<F> {
        vec![]
    }

}

fn main() {
    
}