"use client";

import { FC } from "react";
import { createPortal } from "react-dom";

interface PortalProps {
  children: React.ReactNode;
  element?: HTMLElement;
}

export const Portal: FC<PortalProps> = ({
  children,
	//TODO
  element = document.body,
}) => {
  return createPortal(children, element);
};
