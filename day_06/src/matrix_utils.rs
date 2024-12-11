// matrix_utils.rs

/* 
This function takes in both a matrix of any data type and isize indicies and 
returns the value of the matrix ant the given indicies.
*/
pub fn isize_index<T>(matrix: &Vec<Vec<T>>, row: isize, col: isize) -> Option<&T> { 
  if row < 0 || col < 0 { 
    return None; 
  } 
  
  let row_u: usize = row as usize; 
  let col_u: usize = col as usize; 
  
  if row_u < matrix.len() && col_u < matrix[row_u].len() { 
    Some(&matrix[row_u][col_u]) 
  } 
  else { 
    None 
  } 
}


/* 
Lets you change a value of a matrix using isize indexing
*/
pub fn isize_insert<T>(matrix: &mut Vec<Vec<T>>, row: isize, col: isize, value: T) { 
  if row < 0 || col < 0 { 
    return; 
  } 
  
  let row_u: usize = row as usize; 
  let col_u: usize = col as usize; 
  
  if row_u < matrix.len() && col_u < matrix[row_u].len() { 
    matrix[row_u][col_u] = value; 
  } 
}


/* 
Lets you add a value to a vector in a matrix using isize indexing
*/
pub fn isize_push<T>(matrix: &mut Vec<Vec<Vec<T>>>, row: isize, col: isize, value: T) { 
  if row < 0 || col < 0 { 
    return; 
  } 
  
  let row_u: usize = row as usize; 
  let col_u: usize = col as usize; 
  
  if row_u < matrix.len() && col_u < matrix[row_u].len() { 
    matrix[row_u][col_u].push(value);  
  } 
}

