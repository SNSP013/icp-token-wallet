export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_balance' : IDL.Func([IDL.Principal], [IDL.Nat64], ['query']),
    'init' : IDL.Func([], [], []),
    'transfer' : IDL.Func(
        [IDL.Principal, IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
