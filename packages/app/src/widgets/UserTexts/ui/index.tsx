import { FC } from "react";

import { TextList, getTextsByUserId } from "enities/Text";

import styles from "./UserTexts.module.scss";

interface ProfileInfoProps {
  id: number;
}

export const UserTexts: FC<ProfileInfoProps> = async ({ id }) => {
  try {
    const res = await getTextsByUserId(id);

    return (
      <div className={styles.wrapper}>
        {res ? <TextList texts={res} /> : <div>Loading...</div>}
      </div>
    );
  } catch (error) {
    return <div>{error.message}</div>;
  }
};
