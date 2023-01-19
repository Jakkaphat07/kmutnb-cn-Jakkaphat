fn main() {
    let num =100;
    let mut sum=(num/2)*(num+1);
    println!("The sum of the first {} natural numbers is: {}", num, sum);
}
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_sum() {
       // Set the expected output 
       let num =100;
       let mut sum=(num/2)*(num+1);
       assert_eq!(sum,5050)
   }
}