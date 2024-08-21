/* eslint-disable camelcase */
import {invoke} from '@tauri-apps/api';

export const greet = async () => {
  // now we can call our Command!
  // Right-click the application background and open the developer tools.
  // You will see "Hello, World!" printed in the console!
  await invoke('greet', {name: 'World'})
    // `invoke` returns a Promise
    .then((response) => console.log(response));
};

export const testSetup = async () => {
  // now we can call our Command!
  // Right-click the application background and open the developer tools.
  // You will see "Hello, World!" printed in the console!
  await invoke('test_setup')
    // `invoke` returns a Promise
    .then((response) => console.log(response));
};

export const testDb = async () => {
  // now we can call our Command!
  // Right-click the application background and open the developer tools.
  // You will see "Hello, World!" printed in the console!
  await invoke('test_db')
    // `invoke` returns a Promise
    .then((response) => console.log(response));
};
