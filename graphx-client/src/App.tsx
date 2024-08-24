import {useState} from 'react';
import HomeScreen from './screens/Home';
import GraphScreen from './screens/Graph';

// <button onClick={dosome}>click</button>

// type Screens = 'home' | 'example' | 'random';

function App() {
  const [currentScreen, setCurrentScreen] = useState('home');

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

  return <HomeScreen onChangeScreen={setCurrentScreen} />;
}

export default App;
