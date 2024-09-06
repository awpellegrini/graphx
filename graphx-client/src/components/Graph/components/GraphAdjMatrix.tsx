type GraphAdjMatrixProps = {
  adj_mat: number[][];
  style?: any;
};

export default function GraphAdjMatrix({
  adj_mat,
  style = {},
}: GraphAdjMatrixProps) {
  return (
    <div
      style={{
        maxWidth: 180,
        maxHeight: 180,
        overflow: 'auto',
        // border: '1px solid white',
        ...style,
      }}
    >
      {adj_mat.map((row, i) => (
        <div style={{display: 'flex'}}>
          {row.map((value, j) => (
            <div
              key={`${i}-${j}`}
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
