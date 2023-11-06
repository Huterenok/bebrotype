import { FC } from "react";
import Image from "next/image";

import { getUser } from "../api";
import { IUser } from "../types";

import { Bar } from "shared/ui";
import styles from "./Profile.module.scss";

interface ProfileProps {
  id: number;
}

export const Profile: FC<ProfileProps> = async ({ id }) => {
  try {
    const profile = await getUser(id);

    return profile ? (
      <div className={styles.wrapper}>
        <Image
          width={256}
          height={256}
          src={profile.avatar ?? "/icons/profile.svg"}
          alt="Profile"
          className={styles.image}
        />
        <div className={styles.fields}>
          <p className={styles.field}>
            Username: <Bar>{profile.username}</Bar>
          </p>
          <p className={styles.field}>
            {profile.near_address ?? "Don't have any registered NEAR address"}
          </p>
        </div>
      </div>
    ) : (
      <div>Loading...</div>
    );
  } catch (error) {
    return <div>{error.message}</div>;
  }
};
