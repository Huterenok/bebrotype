import { FC } from "react";
import Link from "next/link";

import { getTextById } from "enities/Text";

import { Title, Text } from "shared/ui";

interface TextInfoProps {
  id: number;
}

export const TextInfo: FC<TextInfoProps> = async ({ id }) => {
  try {
    const res = await getTextById(id);

    return res ? (
      <div>
        <Link href={`/typing/${id}`}>Let&apos;s type it!</Link>
        <Title>{res.title}</Title>
        <Text>{res.content}</Text>s
      </div>
    ) : (
      <div>Loading...</div>
    );
  } catch (error) {
    //TODO
    //@ts-ignore
    return <div>{error}</div>;
  }
};
