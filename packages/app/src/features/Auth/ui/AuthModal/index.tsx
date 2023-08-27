"use client";

import { useUnit } from "effector-react";
import { FC } from "react";

import { $isModalOpened, modalToggleFn } from "../../model/modal";
import { AuthForm } from "./AuthForm";

import { Modal } from "shared/ui";

export const AuthModal: FC = () => {
  const isModalOpened = useUnit($isModalOpened);
  const modalToggle = useUnit(modalToggleFn);

  const onClose = () => {
    modalToggle();
  };

  return (
    <Modal isOpened={isModalOpened} onClose={onClose}>
      <AuthForm />
    </Modal>
  );
};
