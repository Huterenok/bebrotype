"use client";

import Link from "next/link";
import Image from "next/image";
import { useUnit } from "effector-react";
import { FC, Suspense } from "react";

import { $user } from "enities/User";
import { modalToggle } from "../../model";

import { Button, ButtonColor } from "shared/ui";

export const AuthButton: FC = () => {
  const user = useUnit($user);
  const modalToggleFn = useUnit(modalToggle);

  const onAuthClick = () => {
    modalToggleFn();
  };

  return user ? (
    <Link href={"/profile"}>
        <Button color={ButtonColor.PRIME}>
          Profile
          <Image
            src="/icons/profile.svg"
            alt="Profile"
            width={20}
            height={20}
          />
        </Button>
    </Link>
  ) : (
    <Button onClick={onAuthClick} color={ButtonColor.PRIME}>
      Login/Register
    </Button>
  );
};
