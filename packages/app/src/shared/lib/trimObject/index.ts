export const trimObject = <T>(data: T): T => {
  for (let key in data) {
    if (typeof data[key] == "string") {
      //@ts-ignore, TODO: type this
      data[key] = data[key].trim();
    }
  }
  return data;
};
