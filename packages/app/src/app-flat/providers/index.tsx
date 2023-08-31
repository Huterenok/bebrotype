import { FC } from "react";

import { EffectorNext } from "@effector/next";
import { AuthProvider } from "./AuthProvider";
import { ToastContainer } from "react-toastify";

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
