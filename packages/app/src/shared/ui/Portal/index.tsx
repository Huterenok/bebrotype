"use client";

import { FC, useEffect, useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

interface PortalProps {
  children: React.ReactNode;
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

// export const Portal: FC<PortalProps> = ({ children }) => {
//   const ref = useRef<Element | null>(null);
//   const [mounted, setMounted] = useState(false);

//   useEffect(() => {
//     ref.current = document.body;
//     setMounted(true);
//   }, []);

//   return mounted && ref.current ? createPortal(children, ref.current) : null;
// };
