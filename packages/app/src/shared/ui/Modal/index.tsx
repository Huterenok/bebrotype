"use client";

import { FC } from "react";

import { Portal } from "../Portal";

import styles from "./Modal.module.scss";
import classNames from "classnames";

interface ModalProps {
  children: React.ReactNode;
  isOpened?: boolean;
  onClose?: () => void;
}

export const Modal: FC<ModalProps> = ({ children, isOpened, onClose }) => {
  const onContentClick = (e: React.MouseEvent) => {
    e.stopPropagation();
  };

  return (
    <Portal>
      <div
        className={classNames(styles.wrapper, { [styles.opened]: isOpened })}
        onClick={onClose}
      >
        <div className={styles.content} onClick={onContentClick}>
          {children}
        </div>
      </div>
    </Portal>
  );
};
