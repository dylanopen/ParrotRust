pub fn add(a: usize, b: usize) -> usize
{
	return a + b;
}

pub fn addf(a: f64, b: f64) -> f64
{
	return a + b;
}





#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_add()
	{
		assert_eq!(
			add(2, 2), 4
		);
	}

	#[test]
	fn test_addf()
	{
		assert_eq!(
			addf(2.5, 3.8), 6.3
		);
	}

}
