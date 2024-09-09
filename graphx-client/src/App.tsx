import {useState} from 'react';
import HomeScreen from './screens/Home';
import GraphScreen from './screens/Graph';
import PlaygroundScreen from './screens/Playground';

// <button onClick={dosome}>click</button>

function App() {
  const [currentScreen, setCurrentScreen] = useState<Screens>('playground');

  if (currentScreen === 'example') {
    return (
      <GraphScreen type="example" onBack={() => setCurrentScreen('home')} />
    );
  }

  if (currentScreen === 'random') {
    return (
      <GraphScreen type="random" onBack={() => setCurrentScreen('home')} />
    );
  }

  if (currentScreen === 'playground') {
    return <PlaygroundScreen onBack={() => setCurrentScreen('home')} />;
  }

  return <HomeScreen onChangeScreen={setCurrentScreen} />;
}

export default App;
