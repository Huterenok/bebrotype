import { EffectorNext } from "@effector/next";
import { FC } from "react";

import { ToastContainer } from "react-toastify";
import { AuthProvider } from "./AuthProvider";

interface ProvidersProps {
  children: React.ReactNode;
}

export const Providers: FC<ProvidersProps> = ({ children }) => {
  return (
    <EffectorNext>
      <AuthProvider>
        <ToastContainer />
        {children}
      </AuthProvider>
    </EffectorNext>
  );
};
