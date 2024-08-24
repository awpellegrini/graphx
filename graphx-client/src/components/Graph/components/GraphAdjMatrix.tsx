type GraphAdjMatrixProps = {
  adj_mat: number[][];
};

export default function GraphAdjMatrix({adj_mat}: GraphAdjMatrixProps) {
  return (
    <div style={{border: '1px solid green'}}>
      {adj_mat.map((row, i) => (
        <div style={{display: 'flex', border: '1px solid black'}}>
          {row.map((value, j) => (
            <div
              style={{
                color: 'black',
                display: 'inline-flex',
                justifyContent: 'center',
                alignItems: 'center',
                width: 20,
                height: 20,
                flexShrink: 0,
                backgroundColor:
                  (i % 2 === 1 && j % 2 === 0) || (i % 2 === 0 && j % 2 === 1)
                    ? 'white'
                    : 'lightgray',
              }}
            >
              {value}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}
