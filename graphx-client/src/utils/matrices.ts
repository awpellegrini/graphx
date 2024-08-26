// const adjExample1 = [
//   [0, 1, 0, 0, 0],
//   [1, 0, 1, 1, 0],
//   [1, 0, 0, 0, 1],
//   [1, 1, 0, 0, 0],
//   [1, 0, 1, 0, 0],
// ];

// const adjFromWeb = [
//   [0, 1, 0, 1, 1],
//   [0, 0, 0, 1, 0],
//   [0, 0, 0, 0, 1],
//   [0, 0, 0, 0, 0],
//   [0, 1, 0, 0, 0],
// ];

// const A = [
//   [1, 0, 0, 0],
//   [0, 0, 0, 0],
//   [0, 0, 0, 0],
//   [0, 0, 0, 0],
// ];

// const B = [
//   [2, 1, 0, 0],
//   [0, 3, 0, 0],
//   [0, 0, 0, 0],
//   [0, 0, 0, 0],
// ];

// const myAdj = [
//   [0, 1, 0, 0],
//   [0, 0, 1, 1],
//   [1, 0, 0, 0],
//   [0, 0, 0, 0],
// ];

export function testMatrixStuff() {
  // const A2 = multiply(myAdj, myAdj);
  // const A3 = multiply(A2, myAdj);
  // console.table(myAdj);
  // console.table(A2);
  // console.table(A3);
}

// function initializeZeroMatrix(n: number) {
//   let matrix = [];

//   for (let i = 0; i < n; i++) {
//     matrix.push(new Array(n).fill(0));
//   }

//   return matrix;
// }

export function matrixSubtract(A: number[][], B: number[][]) {
  return A.map((row, i) => row.map((value, j) => value - B[i][j]));
}

export function matrixTrace(A: number[][]) {
  return A.reduce((a, b, i) => a + b[i], 0);
}

// function multiply(mat1: number[][], mat2: number[][]) {
//   let i, j, k;

//   let N = mat1.length;

//   const res = initializeZeroMatrix(N);

//   for (i = 0; i < N; i++) {
//     for (j = 0; j < N; j++) {
//       res[i][j] = 0;
//       for (k = 0; k < N; k++) {
//         res[i][j] += mat1[i][k] * mat2[k][j];
//       }
//     }
//   }

//   return res;
// }
