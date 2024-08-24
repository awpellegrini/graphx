import {testConnection} from '../api';

const background = {
  width: '100vw',
  height: '100vh',
  background: 'black',
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center',
  alignItems: 'center',
};

type HomeScreenProps = {
  onChangeScreen: (name: string) => void;
};

export default function HomeScreen({onChangeScreen}: HomeScreenProps) {
  return (
    <div style={background}>
      <h1 style={{color: 'white'}}>GraphX</h1>

      <div>
        <button onClick={() => onChangeScreen('example')}>Example Graph</button>

        <button onClick={() => onChangeScreen('random')}>Random Graph</button>
      </div>

      <div>
        <button onClick={testConnection}>test</button>
      </div>
    </div>
  );
}
