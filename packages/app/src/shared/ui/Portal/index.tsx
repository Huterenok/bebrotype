"use client";

import { FC, useEffect, useState } from "react";
import { createPortal } from "react-dom";

interface PortalProps {
  children: React.ReactNode;
  element?: HTMLElement;
}

// export const Portal: FC<PortalProps> = ({
//   children
// }) => {
//   return createPortal(children, document.body);
// };

export const Portal: FC<PortalProps> = ({ children }) => {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);

    return () => setMounted(false);
  }, []);

  return mounted ? createPortal(children, document.body) : null;
};
