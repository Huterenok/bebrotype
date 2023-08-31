"use client";

import { FC } from "react";
import Image from "next/image";
import { useUnit } from "effector-react";

import { $user } from "../../model";

import { Card } from "shared/ui";

import styles from "./Profile.module.scss";

//TODO: change name
export const Profile: FC = () => {
  const user = useUnit($user);
  return (
    <div className={styles.wrapper}>
      <Image
        width={256}
        height={256}
        src={user?.avatar ?? "/icons/profile.svg"}
        alt="Profile"
        className={styles.image}
      />
      <Card className={styles.card}>
        <div>ID: {user?.id}</div>
        <div>Username: {user?.username}</div>
        <div>Email: {user?.email}</div>
      </Card>
    </div>
  );
};
