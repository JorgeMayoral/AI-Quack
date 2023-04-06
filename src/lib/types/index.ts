export enum Role {
  User = 'You',
  System = 'Quacker',
}

export interface Message {
  role: Role;
  message: string;
}
