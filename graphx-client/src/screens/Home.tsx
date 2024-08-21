import {greet, testSetup, testDb} from '../api';

const background = {
  width: '100vw',
  height: '100vh',
  // background: 'black',
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
};

type HomeProps = {
  onSelectGraph: () => void;
};

export default function HomeScreen(props: HomeProps) {
  return (
    <div style={background}>
      <div>
        <button onClick={props.onSelectGraph}>click</button>
      </div>

      <div>
        <button onClick={greet}>greet</button>

        <button onClick={testSetup}>test setup</button>

        <button onClick={testDb}>test db</button>
      </div>
    </div>
  );
}
