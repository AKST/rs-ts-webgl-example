import * as React from 'react';
import { checkExists } from 'base/types/types';
import { useSingletonFactory } from 'hooks/use_singleton_factory';
import { RuntimeModule, Runtime } from 'runtime/types';

export function useRuntime(
    canvas: HTMLCanvasElement | undefined,
    module: RuntimeModule | undefined,
    width: number,
    height: number,
    onError: (e: Error) => void,
    vertShader: string,
    fragShader: string,
) {
  const getContext = createContextFactory(canvas);

  React.useEffect(() => {
    if (module == null || canvas == null) return;

    module.setupPanicHook();

    const builder = new module.RuntimeBuilder();
    const context = getContext(canvas);

    try {
      builder.linkWebglContext(context);
      builder.linkVertShader(vertShader);
      builder.linkFragShader(fragShader);
      builder.debugState();

      const runtime = builder.createRuntime();
      runtime.debugState();
      runtime.tick();
    } catch (e) {
      onError(e);
    }
  }, [
    onError,
    fragShader,
    vertShader,
    canvas,
    module,
  ]);
}

function createRuntimeFactory(module?: RuntimeModule) {
  return useSingletonFactory((module: RuntimeModule) => new module.Runtime(), [module]);
}

function createContextFactory(canvas?: HTMLCanvasElement) {
  return useSingletonFactory((canvas: HTMLCanvasElement) => {
    return checkExists(
        canvas.getContext("webgl"),
        'webgl is needed to run this',
    );
  }, [canvas]);
}
