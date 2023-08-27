"use client";

import { FC } from "react";
import { createPortal } from "react-dom";

interface PortalProps {
  children: React.ReactNode;
  element?: HTMLElement;
}

// TODO: document.body must be only on client
export const Portal: FC<PortalProps> = ({ children }) => {
  return createPortal(children, document.body);
};

// export const Portal: FC<PortalProps> = ({ children }) => {
//   const [mounted, setMounted] = useState(false);

//   useLayoutEffect(() => {
//     setMounted(true);

//     return () => setMounted(false);
//   }, []);

//   return mounted ? createPortal(children, document.body) : null;
// };
