function det(matrix: number[][]): number {
  if (matrix.length === 1) {
    return matrix[0][0];
  }
  if (matrix.length === 2) {
    return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
  }
  let determinant = 0; // Renamed to avoid conflict
  for (let i = 0; i < matrix.length; i++) {
    const sign = i % 2 === 0 ? 1 : -1;
    const minor = matrix.slice(1).map((row) => row.filter((_, j) => j !== i));
    determinant += sign * matrix[0][i] * det(minor);
  }
  return determinant;
}

export function rank(matrix: number[][]) {
  const numRows = matrix.length;
  const numCols = matrix[0].length;
  const minDim = Math.min(numRows, numCols);
  let rank = 0;
  for (let i = 1; i <= minDim; i++) {
    const submatrices = genSubs(matrix, i);
    for (const submatrix of submatrices) {
      if (det(submatrix) !== 0) {
        rank = i;
        break;
      }
    }
    if (rank !== i) {
      break;
    }
  }
  return rank;
}

function genSubs(matrix: number[][], size: number) {
  const submatrices = [];
  for (let i = 0; i <= matrix.length - size; i++) {
    for (let j = 0; j <= matrix[0].length - size; j++) {
      const submatrix = [];
      for (let k = i; k < i + size; k++) {
        submatrix.push(matrix[k].slice(j, j + size));
      }
      submatrices.push(submatrix);
    }
  }
  return submatrices;
}
