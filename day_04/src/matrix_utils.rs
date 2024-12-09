
/* 
This function takes in both a matrix of any data type and isize indicies and 
returns the value of the matrix ant the given indicies.
*/
pub fn get_value_at<T>(matrix: &Vec<Vec<T>>, row: isize, col: isize) -> Option<&T> { 
  if row < 0 || col < 0 { 
    return None; 
  } 
  
  let row = row as usize; 
  let col = col as usize; 
  
  if row < matrix.len() && col < matrix[row].len() { 
    Some(&matrix[row][col]) 
  } 
  else { 
    None 
  } 
}