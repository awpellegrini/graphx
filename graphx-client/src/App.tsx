import {useState} from 'react';
import HomeScreen from './screens/Home';
import GraphScreen from './screens/Graph';

// <button onClick={dosome}>click</button>

function App() {
  const [currentScreen, setCurrentScreen] = useState('home');

  if (currentScreen === 'home') {
    return <HomeScreen onSelectGraph={() => setCurrentScreen('graph')} />;
  }

  return <GraphScreen />;
}

export default App;
