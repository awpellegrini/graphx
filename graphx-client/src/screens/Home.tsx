type HomeScreenProps = {
  onChangeScreen: (name: string) => void;
};

export default function HomeScreen({onChangeScreen}: HomeScreenProps) {
  return (
    // @ts-expect-error
    <div style={containerStyle}>
      <h1 style={titleStyle}>GraphX</h1>

      <button onClick={() => onChangeScreen('example')}>Example Graph</button>
      <button onClick={() => onChangeScreen('random')}>Random Graph</button>

      <button style={playButtonStyle} onClick={() => console.log('ok')}>
        Playground
      </button>
    </div>
  );
}

const containerStyle = {
  width: '100vw',
  height: '100vh',
  background: 'black',
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center',
  alignItems: 'center',
  gap: 24,
};

const titleStyle = {
  color: 'white',
  fontSize: 64,
};

const playButtonStyle = {
  width: 240,
  height: 48,
  fontSize: 20,
};
