"use client";

import { FC } from "react";
import Image from "next/image";
import { useUnit } from "effector-react";

import { $user, updateUserFn } from "../../model";

import { Input } from "shared/ui";

import styles from "./Profile.module.scss";

//TODO: change name
export const Profile: FC = () => {
  const [user, updateUser] = useUnit([$user, updateUserFn])!;

  const onChangeUserData = (e: React.ChangeEvent<HTMLInputElement>) => {
    //TODO
    if (user) {
      updateUser({
        ...user!,
        [e.target.name]: e.target.value,
      });
    }
  };

  return user ? (
    <div className={styles.wrapper}>
      <Image
        width={256}
        height={256}
        src={user?.avatar ?? "/icons/profile.svg"}
        alt="Profile"
        className={styles.image}
      />
      <div className={styles.fields}>
        <Input
          onChange={onChangeUserData}
          value={user.username}
          label="Username"
          name="username"
          placeholder="Your username"
        />
        <Input
          onChange={onChangeUserData}
          value={user.email}
          label="Email"
          name="email"
          placeholder="Your email"
        />
        {/* TODO */}
        {user.near_address ?? "You don't have any registered NEAR address"}
      </div>
    </div>
  ) : null;
  {
    /* TODO */
  }
};
