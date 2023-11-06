"use client";

import { useGate, useUnit } from "effector-react";
import { FC } from "react";
import { useRouter } from "next/navigation";

import { $isModalOpened, modalToggleEv, routerGate } from "../../model";
import { AuthForm } from "./AuthForm";

import { Modal } from "shared/ui";

export const AuthModal: FC = () => {
  const [isModalOpened, modalToggle] = useUnit([$isModalOpened, modalToggleEv]);

  const router = useRouter();
  useGate(routerGate, router);

  const onClose = () => {
    modalToggle();
  };

  return (
    <Modal isOpened={isModalOpened} onClose={onClose}>
      <AuthForm />
    </Modal>
  );
};
