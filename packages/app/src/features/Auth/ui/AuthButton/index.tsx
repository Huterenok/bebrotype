"use client";

import Link from "next/link";
import { useEvent, useUnit } from "effector-react";
import { FC, useEffect } from "react";

import { $user, modalToggle } from "../../model";

import { Button, ButtonColor } from "shared/ui";

export const AuthButton: FC = () => {
  const user = useUnit($user);
  const modalToggleFn = useEvent(modalToggle);

  const onAuthClick = () => {
    modalToggleFn();
  };

  return user ? (
    <Link href={"/profile"}>
      <Button color={ButtonColor.PRIME}>Profile</Button>
    </Link>
  ) : (
    <Button onClick={onAuthClick} color={ButtonColor.PRIME}>
      Login/Register
    </Button>
  );
};
