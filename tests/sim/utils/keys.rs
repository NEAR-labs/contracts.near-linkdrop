use near_sdk::json_types::Base58PublicKey;
use near_sdk::serde::export::TryFrom;

const KEYS: [&str; 200] = [
  "8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe",
  "HYE4EaDGTFJvDFA7BD8GpkHbsWga5KB48xUckhuSC7mz",
  "HagwDtL84Diw5SUg4TkLdoz5UAexucxzv71JsuFcF4k1",
  "EwoBJq9V6oTfkioAKw1gAapMkMuqGwUXeyRvFHCZkgM9",
  "BZ4VoL3KQ9juT3zXt8ATVDUobLkYFUKfrSzBCawYaYpR",
  "7ARGTd4eaP593J8XsySr8AHdYqUJShf54EC6jUgpKF7K",
  "GuEsrHr4cnbP2QBdLZcMsa3bPnCnmWcz8yJHAfuoi4GE",
  "8YgQFr37QbPpxuSySs8uaZ7nA1RNyu82pRd5veU7LrFm",
  "3FdfTzBUcjsNdweajixiDRyaMRgkmjDyW514VRcyUNox",
  "FnSbwdC62eGmwBfuvckpXjasZXbqTCyQHYKarCXHi39c",
  "8fpbV3eDqUA1X114t7zseLkxFeTH9CJn7D7r42RfQJXx",
  "FtWskm4kAkUCRTaxMtEuLtPrkfg9mtF7644MGLkbFQhL",
  "8xwkeiRnpWocNB2QSX4yvqDEpqEECSVzHdU5kHyTtKG9",
  "FdWjTwn6A3xXWFrLSjQMkB27MQ3MDuYyHqnhxu57pY1d",
  "84caYnECvButihCePh61rFZgm3QkA5yWtjmBK8VB4dn6",
  "7Q28RdnwYHXf9AeNs9E81E5EYsJyBSMMVehepkBGeo8j",
  "9uaJyUeQXMDGZxbowr9dVMrwvcAMbCWT1BH882AhWNBF",
  "5fzkSuDY6EgngsJoHaecaak5BYA1dHLWD5vdGPzkAVYU",
  "CzHrMvgcBsugSFtZuDG6SpkFugD2MtomAHrJZQ5RBRhP",
  "H2dytLXjSJosUjad6b4BknGznCMC75XiVJPurgsx4GQW",
  "5BMNthL3dBCNjAa6ppSDkEiG6r7xFhKcX6xKpU1Rzver",
  "CKV4shP2mkib2a73iTgpJp3BwFRZL5Mk6a9tUkMsGChP",
  "3j7uGcxqfzut1WaH1BVmwj99tmcjpa4gVe6vATfXuNFz",
  "vbitWbhPjAadztqFL1CGAMF5jnjpFiPwo71sTLGpZgb",
  "3o2wDLBscftHDKCovRhabood47vuLrBdYNDDxPHo2XKD",
  "Ez7Y8HX6LnzNBQkL6cmR6MhcoC2K4RTJssH9dt2wW6Ef",
  "BHuzW3GohRCeo6uRgt3CnzMXXKhhUeCZ6PNFqBxcZw8w",
  "BmQJYsp9pGtpLh5Htrn5FCWunnUMJaEWqSgmiBjXSjcM",
  "967pc1cBbsy2NxPmQmt3fPknBoPeb4rRWnEx6Vn64Bjo",
  "6QhhrQ3UeBDpTSjBZr3vg1CxBM8Buv4piW5Lw4mYwv6K",
  "6KMiBVtfCqgH5pChgmXsNnyVnTFPcdSB4YV9f6WdF9pT",
  "Av63U7APaYVSkXcAYt2LRdNQxwDVndXDKjTwAACdPPu5",
  "6JMrPi9xgtN8vU4XNvJBw6HMbVN3gtjK8GcVGiUh6yFn",
  "AAKwNw99D3TK2MzBMqPxiusEsdJtDb8JAhQL1Wds7hYo",
  "2R8TFRb43fewY728YsjqJPAjTZUxj629RmGLcdMLijkp",
  "56mmMRBDBqdefaub5ptMVHzJGc8YdgUiRLXLv6V7ZSx2",
  "4Re1N7acMSxMYHHQv27rSBS1q8YKQozDAknUsCptr9HF",
  "3aoKUZBFT2TK5WsmqnUbDRchEN7pZnGfuVsYafFfhK9x",
  "5ouBq4etuKSCk9xKQtcugRByttFwjQv3rh8xtDzn8J58",
  "GBNqt8pGW8kqcKag1JGUNyHFRHiogrsPN8BSRBu6c5ee",
  "CwPLj9kxTGzDSqbEmxURGzcG12DvqWG9ciyHaDihEZm5",
  "EuA55gBoWtfzLp8yW2fjYPxGK2fTS9aryhLVaYR41tYT",
  "2C3TgMGcsYSCjoD6Kywz9GLuBsHgmARes1h5uX7KA2fK",
  "AvXvaT7AehThJBKZA6qBtyA6G7ZHTdGdUNF1RiFQuNcm",
  "AH7PfonkyzpDp1Y5K6T8iqSrmCTXJ4HKV42tQz6dEXEw",
  "7zxAC7vPGq3RQt5hibbZGj4tZAiiomX8UYxcr43QHRCh",
  "2pxvpLY5d6Hxtc2RW9MkaJA33EifuMRGKU447wm6KxP3",
  "C8YQ7eGdQppWXGMbuvgZTCJuhD9LxCXHNAiV1hnMyZZw",
  "y1CGCF9VMWNdhEVpfnueVUoPaqsYRE6dU7FSDW6VEw4",
  "3D6h2QBysM2rD8fVyHzGVTLPFwAJc3jKC14BccrKd2hX",
  "CGUsZgWGkYmA8nfhsYGEHGgL9QV2Cp3XpCo5trZGcGvK",
  "9MmhiftpxsNeSSWZFmRcWBgNVKJJ9q7z7rPhQXunJwmi",
  "4tV2yaBYuuKA2i98wV22r4v1AVF7xJtuVetvDhQK6Eo3",
  "9hZxbJ5sMSitAhwSHNaf4rnfqSa3sBVdv7viQxg3mBfJ",
  "CtFoCVpjiE3yZbTXQxFacvNn1oZ8Yuzrjuo2WiyUfx5L",
  "8NruGGQy3pX1EGU6HK4LJqLDfLZZ6anWkskzZdhyi2pL",
  "Cv6bN24TFUzDXhmKpM1ZP2y7D9fCPn2BrmX1skVRtPDU",
  "3RgsRAK2VfoJJFuAfV8ZePmpWLJMEoGvyYHuBPwLq2ee",
  "9GqhMAvtNYXQvjh2MLsb5ofFNhcYaArQ5odxRyk6BXBS",
  "BofN4Qntz8QKPRjE9UEDYoogSyFf7kBmXrFfGcsnBAVC",
  "UxGdEuX9Lpn5w9mQKCXqYN5DuxUoYfsPXVThjvfdbov",
  "8MXX3nfLrDwVRQHoMw4k47JjmqcUQ63rHuAqKtZBk4rN",
  "nqPhv7gHzoZYDuQ57jw5DyH9yqs38NjtVvHLpXMzDhP",
  "3dtfa49QiozCHfer2UpNppUASKuMiAXUMFpGsXPwWufj",
  "392BnwhNxmyDtRTBd25ta3GE23RZj6bp8575gNc7QKq8",
  "CHo1F8mB4tJTZja6pPerqP9pad8AR84XzD79Xr4BBQcN",
  "AjiW28NVYooS9Wep9t2ZpAmSXvKur492LemPTnyF5a8X",
  "3YSbhKYUaiL296pCNXCJpKMQZ5ivuHLcBRA7up95zFug",
  "4G13EKUHxSnyq7UUoLmSQteYJ1uB9TqMmjdFo9UndEYZ",
  "U1ASwRNRZveqB1i6jLo2XqyaqY47L13EFjMUCiJEVLk",
  "25SswZwU9mv2dNzJBtgcVdBm2KhLXVqSvC8XjSW7Mfgq",
  "GjiQ9rPs9XV6KTEww9B46GLgECQT1GADE62KVA5kDYFT",
  "GaLovyyy3FFodg1KaiZPtEa9kKhLNgfoznJZqdfHAxQD",
  "83L87EDzFUgRH8nkrB2KP4b9trYEUq87q1iP7cEvVuXo",
  "CXSo8HeTCkEyMSeubjnHRW3Mr6t1uxba2tnHSMEAY2sB",
  "J5yGY4T8yBNmsppVtBWVJkpi9Zh2gQ3L3pWYnu9fvFeY",
  "G8rUgoBQjqW6E5cCaVYYzU2vKw2VyVm9hGAyXfkRbngA",
  "FKTr1dJidCaq5EhyrphpDEEEm3PZN8PsJMBXMUadVF8S",
  "AUGhurJvFjFqYhDEQ1aZgVZyw2hjPH9JtKmqJQZRAUfX",
  "9Kzn4EwrVp6BmxXn7D1HN26NSoHge3ndpFCsgMRVuwHJ",
  "D92XmdctMKh1vLD3fefTtJoVmxR6SH6LffvcEEAreLEm",
  "2FcMedKCrpF4cNxjhnJhtUMegJCkk2Qxg1SxKk6dKJAw",
  "9wEnS5V3tFsYs7LJNvmYCtvgVbk5qaBYmJVcQiajkyWf",
  "HJu7UeZs5ivXszycWYkwQhgWL1Ur4pgDMdxkq3ZAdaSh",
  "4e3wyCq4yW15pDHtjGm49kdy7UkFgWh7vp8MaWGvZ4uz",
  "7XTYZZdSPYpUwQagTMmZhqopjBooKy8opZq3jYryUPcb",
  "7CS9vnpXsyHEbaq2kq17YXbzykZ4ep1Gt5rj4sfGSitQ",
  "BwAA2cDm77N9SdJhLxNAWNQhauxnu7BhfuUjwu9e6vcL",
  "5UpCgXAFJqvnvQxRQpnq5b88PxueCJBjjsFtbXQESVjS",
  "9CrnaxRHqRNFEpWXwfnpqaPcYvK7hArLdeQbNMyp8Zzh",
  "3tQ5DMngrtmr1NBoMJsjDnLaeJc2RqqVbyUwnUB5y8EC",
  "3M9Yztb33w8nRAYtGPuLByEdxtDLLeN2nfMRjPYdpaBv",
  "5i9Y4CQBzcjV28EEurF8Qgq9va2k2aGDfBLfvYJhqc7d",
  "JB6X8hPLUC7KTNPnkxHUsHDZAdUTahENKBmoFkxV8gFb",
  "9B34kMfD38zzVmHs5eHEPqXo9ucAGijSK182CobaKTDU",
  "8RTyEbG7He46QUxMwbaGwEEEf2qy9mrY2gpScD2thSVB",
  "HhwXgmjBrZyCanj1f8GJTFNZuAFNqVnTyxWS7V3uwdKy",
  "ADaXyVkujd9DBokScJGnVJfcQAh8hYYdjW41X4ELLYWJ",
  "51ijrBNZBWaVyJ1PUhDZYQDnj6ZwNR2i33RrPxNdMEhy",
  "AUpdiPcnLwS92c183RPgJneFJWcQVVpPokbZNqtgxsRf",
  "GjzbQBWF4dsacmYKCDDWbQQZSzxeDw9ZqovjNZM159NJ",
  "4EpNvEwavdfzw4XAoqPsUxS3jnrYJPNv6b4s7kpdT22D",
  "58XwVuZNPu32hLEyroj4wQZVzRqQPi2rP9pZc2n6mssf",
  "AR82AqyD2iqhBbQouqd1CXEM2BSvTRXZgQVW9xrc1rXh",
  "4U6F5XYnSCdNPgdaH4NgcDTQreyHgm9VUUCYoXChmpD9",
  "H7Dtd7FgVaA8f49YPi4b88ttnwZedZJhN6fvYsH6F6ob",
  "6rPzVnwDeMPtumefTSMeJVpEdWxDASpvaBBSFdqLe5AE",
  "2Svvd6yTAWEKrkPDr2afLkgUrzDtty3VzK3nCBihfpz2",
  "6NLAkGtGG2Rh9xy1AaLrKMkNi1VxXYGSJGzfJpQhQYD5",
  "DHnF2dcpNEmBWZopwd2RmDKdM6hspUopQngrKVTPQwYp",
  "8nTe8sB9uSNKmN2fnQuQJNSHRmgdWuWPeYheQbPfPf8n",
  "ECVRG5pQnqcREScvmy9nb6iW6s8tuwbF8HrxT16STQ6h",
  "3UWSvQDzMQbyVpmDbC2h5McHmWbf95TE2HT2KDqW3fSN",
  "9zKGMa9X513ZfH2v1EdrmyjXsUBYWX1ws3qHhjHmaczJ",
  "8b1NRXTuN9bGouDsmjbU1pikRNZqJHdVLhpuTPs6sk7y",
  "8eLG2jZYT5FSz98KZxQxFcWFJ9R5ePwFVW4XUpz4mWFp",
  "4owa2Hu9m9bHXjxEcZVqumtAxxPJroqTUFt3Ee426AsH",
  "3pQwGfGJ5GS5tviL6wZofzZjUdLE3WmQXK7kwD29wMNh",
  "CQ6NPCcZCWuQMQPRuCLXnEZfHKe9Xmhb6JSU2D8mmPns",
  "6uBkuq7cFPL8dKCDCurirf9u2UAKRigyz1yj64pGzVUP",
  "DsgAKM7KwzsLZx6jsByAnrq9UUFzV2nJsBN2BG7LRfjM",
  "61L2gZ9uAygKyeurroCuUvAv4sk3wLdFtzuNZPB73niC",
  "GcLYAedmnzg867BL8zE9a5BME2hn14mYAoVEkYKqFkn3",
  "6SRgN3hRLZP7asKg8U3WM9378gM2Fwi4JdAjKf2criEk",
  "A54oxsBt3dZJ2ZiyDp4D8T24LERyYjUf8wJ6ueG2gaKG",
  "D6iMADmt8xHtfMPneggT1pXzAAWn3XtM6BxbsKysh8JM",
  "Yry3Dx4i9XP9qDfK2DrTtNHG9LkHpe7Kd27aFWD9T93",
  "DfJArudf8b85su4bZtX12JzsGjwwbuZLikHgdced24VT",
  "EVQzQLwPsGja5szDuiL1SfX47q4vGNkPHVSuqEcFoC2x",
  "91BXzv5XGyvSjd6V1tyoWMM1cHPdR6XYBeavsvZvKJBT",
  "5huQ6XgPm2VPL7XFQb57CKrfF5tngwGZuR268gCB84S2",
  "AvZFLhJdAcm77gYRdWcEou86K4Cy1kCVoahEmhPvFkTd",
  "C1Sg2KuBqRF5kagwRVqwfWnL2PQgPnDn8g82wjFvv6u1",
  "3o7EmMsZrp99L5v2nCmUnMxjtWH7n6dHucaoHnkgdtm7",
  "9KJcqtSGeiB1PXCEhammBie7rKm6iXQDCNNqRwT5yfPs",
  "Gbef47z6tzD7EFxBNW46ciCWtD5mvv4EFt1v2evgnh4r",
  "49tyv3JX9ZDgWxmGifRQnvQsZX4DVjfRjs1Ju2uSTqdH",
  "618zvLAt9bEW9VHhB1CPLhfUP7qT5dkd21e7xvcy725d",
  "A7hb7L73k5MgLYNiDyrNLgvhwRazgTNvarv6AhEQ3BWR",
  "DPqp2qrxZeFJqxtHyVqUpEgFsq2gujueKU2pJ1JKzjDc",
  "8CaY8tri6UCoSSQNBcQggtULoZJg5MnrBg6VCBtG4PtS",
  "EWhgPfkw4gMcwCMd9he65oeAawfD5VjLJPDLVcFmrvCy",
  "5yDVGwKzpKHcs4mK6UQu3dvD6cCCFAkgSdoocPVu9ftF",
  "491BL4mCXhwi8cF9eQRhgGNwJm6CFE9XTxYR1AyQhi32",
  "7mc1UGhPLU56Ha1J4za24CMX9BqbvQBAJBeTUd75f2F4",
  "3pY8nKok4Wv5oRW7ufSfHCN2XNaXoqdBvBJ5weUjpiUp",
  "7g6Fxgu3JHQTiB5u5JKx4jwZ2KYFf7g8AiR2MUaW3aeW",
  "G75NyFuqgHjifqSShhguV31BJV9A6bXzYs4uyeANXWkq",
  "8VtGtAcDB4qXUhpzhQS2s11VtmTiEzysK87dd2oN8xgR",
  "8tMyhVqe79nuFu5ymJ7qovmen32bVnFnS5kcgRYmBg8j",
  "BK1wYiyCHC7k5Tx1hksHyXvBKULcCggtUftF6sWYUXso",
  "GfZ43axxdzmCXLrYCJxu1qiL3ADbNLJBPKSg1f1TYZYN",
  "2BSh7jEnC63RHTwq2NDA73XNUrJKKb9CRwo9cwQ5G4bw",
  "FdwbzHZcSF776ZY2QRBMbM7LgvwArgCVc6JgUcaD5wnD",
  "6ye8fpzXGYa23yDEZJwcsj8ruFmtynbrzFPb7WwN64HQ",
  "CR7vcZngpgL2LDhfrpCVU7EHcwS1tL34t8uMR6BGjBYq",
  "B1yRHmR3MvM3n6twktGSqcoSnwvo8EDTsE1ubnfrT5Yt",
  "D1V1uNzJSXWPBzw1nJ6KRT5A1afreGMvfCWxsk6ttK2x",
  "7GgDo7HKQ5qjpNGfLRefuTs5z3KAciEC9gnDn2CHXtZc",
  "FxtaugJtCPSqYFN4WSov4Hnfm6Dne2ZXrPXRn1zG4ZfZ",
  "37q7nhYzinBitBvo8fSmDnKmBSUCtSiqyjz1eUjsPFxU",
  "BgWdMPawHBTVqu2JExe3gK7wvT3VcMsx8gMQg7aaD4gW",
  "BMve1BMPAfuTTMzRTwTEXVVRWvstUofFYwECPzs8qNtd",
  "Q7Pm3pPqjUnvV1kPuYHvaCWavPvBhRW6qewwyQnEWiB",
  "7Po28gvthpXdKpp8jn1GnTZmSfudAR7D2EQu2y8CvLdb",
  "7dWkAkghLjRjqZbJCoWDBhqJAbhDKYDuy37cATVEXNwP",
  "GMG8oxyr65pzv82QWqYPoByMtAfcMkvXecuA4BAp39Hv",
  "2PrV2zBWfSJHEGq8bbh3iGgmwVEv29qKab5kvWQeGYwS",
  "GZ85Qy582pC7D5ATgTVhXqJhUy7Jp5sU6cWm2SEUc1K5",
  "HXR4eHSP2Ue62xU9pasHsBLN2n7ZQh2ABH7TZdYVqEQq",
  "7hNuZaMTfLC9nSpr4WMBYqazVEUr3nZBWYGp2Cd4JUaB",
  "DfPr3BuqdTMsu5RsVGpGoJ8JdZjGsC3MyXyAcogRdL7R",
  "4jLePNMQkoA1Sq7Gc7FP8Nck2jqMDydkFuQa6RaFTaDD",
  "4FUJsnXC44Z4U2MoGPvwYNvKUEk3Deft6V3BYCqRd9pm",
  "7x9ySSTb43spFNQoPQCeBJnjSg511qxokGnVnCbvDrHJ",
  "67eWTJ7sfpz27HdUYwmDGsWyvLSHfteF3sBdcqiMc1Mo",
  "prpH74DsxFEKN4Tx6LPZrVXCFrSEKx83wqxcv1Ey6di",
  "82WMq63S8jpTEK1w2kjzSBvw1AvGVtodhgnk1hpBxpfZ",
  "EJ74yGzShnkjhDv75Nv5BP2TAapooe47S9hVcuC6Wu3U",
  "8VfVBhTqBB828wpUSJ1D5o1qYgBRKDT6FsRaPtRcv1M1",
  "AtXRWdNG3DVj7kuTEa9g3wNh9EWGiZFBdoioxLEKzmYc",
  "HawP6Wif8yRFwAxcLRTdCuRDQix23XtrrqnPJVXtrWNm",
  "FSUYRrJ5duTzah1MWVELZDddWsepGepLN8jRXvsEedvG",
  "HU1UCMpz1YA68HHfxavwm8JoWDLJwGpLykrre3bq6rCN",
  "FKcDbyRXbEgCwWRie1iv8G3jzZAk41rm6LU6TrRNc33A",
  "CNYMcLXXPVHh8jPp5nM86WvUPmvmRZtTZMLXZWrhWZU4",
  "7mGkaPpuvYVVsRKjDvu8KfGXWmbeYbFWC5W7Di9UbH3g",
  "3nxZSkVqSQ1HU8PqmvHV46JSwoAoEpDz6RAYNfwvNCLG",
  "DgZpDZE1UBZxgzJTH4ud2aCX1FMj3qAkdrSiKQ62SSd4",
  "EveuiovFoJZWo9trhP3e4X6bcsDPrKRxZQNaJrag1HEm",
  "BbHogutNdqPsacmkuWv7qerwr1KALdWfZdHBJthdvaz5",
  "2Sv1obsrfw1FF8CGgc4NNQBHCD7K6DZm9M7yaqcMj6t2",
  "FXvSioqnYbZcpNtWkLcTWUWCRqc1BUjxYDjxt9h1ksMD",
  "95LAsKB1KdnjiGf8ZMbcRz77DLKDV2zWudy5yGk7azsb",
  "DrWRvs7VBcYGW72SfNBVYW9yv95VWuyhq9qRTwPCvuBb",
  "4Mk6UKsq4ere9uv8GJMTdSCqFaJUcvYsvAjJ5xC2pqCY",
  "BdxWubFS3StHeg7odLu37vGJZaqPAJpGQL6uL69kDuCy",
  "AeRgJwszf4fTTFmfiyUFdsuUnHhHRmxxcjKWba7mU8hL",
  "6dkLAijj2NGucVzeBsky3JuND3iHWDYDb4A3bV7GxGTn",
  "AwKrBcAU3tPxzSKNx1th82MpTptCCihVd8pGvhw7kdaq",
];

pub fn get_public_keys(from: usize, to: usize) -> Vec<Base58PublicKey> {
  KEYS[from..=to]
    .iter()
    .map(|&key| Base58PublicKey::try_from(key).unwrap())
    .collect()
}
