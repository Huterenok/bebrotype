import { attach } from "effector";
import { createGate } from "effector-react";
import { AppRouterInstance } from "next/dist/shared/lib/app-router-context";

interface NavigateParams {
  path: string;
}

export const createRouterInstace = () => {
  const routerGate = createGate<AppRouterInstance | null>({
    defaultState: null,
  });

  const navigateFx = attach({
    source: routerGate.state,
    effect: async (router, { path }: NavigateParams) => {
      if (router === null) throw new Error("Router is not initialized");
      return router.replace(path);
    },
  });

  return {
    routerGate,
    navigateFx,
  };
};
