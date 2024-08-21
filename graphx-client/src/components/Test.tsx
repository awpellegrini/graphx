import React from 'react';
import {GraphCanvas} from 'reagraph';
import {SIMPLE} from '../mock/graphs';

export default () => <GraphCanvas nodes={SIMPLE.nodes} edges={SIMPLE.edges} />;
