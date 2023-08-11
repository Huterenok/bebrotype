"use client"

import { useUnit } from "effector-react";
import { FC } from "react";

import { $isModalOpened, modalToggle } from "../../model/modal";

import { AuthForm } from "../AuthForm";
import { Modal } from "shared/ui";

export const AuthModal: FC = () => {
  const isModalOpened = useUnit($isModalOpened);
  const modalToggleFn = useUnit(modalToggle);

  const onClose = () => {
    modalToggleFn();
  };

  return (
    <Modal isOpened={isModalOpened} onClose={onClose}>
      <AuthForm />
    </Modal>
  );
};
