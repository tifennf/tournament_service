use crate::ressources::Pool;

pub fn make_pools(number: u8) -> Vec<Pool> {
    (0..number).into_iter().map(|n| Pool::new(n)).collect()
}
