import * as React from 'react';

export type Dimensions = { width: number, height: number };

export const ViewportDimensions = React.createContext<Dimensions>({
  width: window.innerWidth,
  height: window.innerHeight,
});
