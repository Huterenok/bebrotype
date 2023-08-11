import { EffectorNext } from "@effector/next";
import { FC } from "react";

import { ToastContainer } from "react-toastify";

interface ProvidersProps {
  children: React.ReactNode;
}

export const Providers: FC<ProvidersProps> = ({ children }) => {
  return (
    <EffectorNext>
      <ToastContainer />
      {children}
    </EffectorNext>
  );
};
