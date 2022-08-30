#![allow(unused_imports)]
use graphql_client::*;
use uuid::Uuid;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AcceptQuest.graphql"
)]
#[cfg(feature="AcceptQuest")]
pub struct AcceptQuest;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ActivateSpecialItem.graphql"
)]
#[cfg(feature="ActivateSpecialItem")]
pub struct ActivateSpecialItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminGiveItems.graphql"
)]
#[cfg(feature="AdminGiveItems")]
pub struct AdminGiveItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminGivePet.graphql"
)]
#[cfg(feature="AdminGivePet")]
pub struct AdminGivePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminLoginAs.graphql"
)]
#[cfg(feature="AdminLoginAs")]
pub struct AdminLoginAs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AdminSendCustomNotif.graphql"
)]
#[cfg(feature="AdminSendCustomNotif")]
pub struct AdminSendCustomNotif;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AnswerGuildInvite.graphql"
)]
#[cfg(feature="AnswerGuildInvite")]
pub struct AnswerGuildInvite;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AnswerTradeOffer.graphql"
)]
#[cfg(feature="AnswerTradeOffer")]
pub struct AnswerTradeOffer;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/AwardTrophy.graphql"
)]
#[cfg(feature="AwardTrophy")]
pub struct AwardTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BidOnAuctionItem.graphql"
)]
#[cfg(feature="BidOnAuctionItem")]
pub struct BidOnAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyGigaItem.graphql"
)]
#[cfg(feature="BuyGigaItem")]
pub struct BuyGigaItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyGigaMatterCheckoutLink.graphql"
)]
#[cfg(feature="BuyGigaMatterCheckoutLink")]
pub struct BuyGigaMatterCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyVendorItem.graphql"
)]
#[cfg(feature="BuyVendorItem")]
pub struct BuyVendorItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/BuyoutAuctionItem.graphql"
)]
#[cfg(feature="BuyoutAuctionItem")]
pub struct BuyoutAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CancelAuctionItem.graphql"
)]
#[cfg(feature="CancelAuctionItem")]
pub struct CancelAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CancelTrade.graphql"
)]
#[cfg(feature="CancelTrade")]
pub struct CancelTrade;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ClaimGigaMatter.graphql"
)]
#[cfg(feature="ClaimGigaMatter")]
pub struct ClaimGigaMatter;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateAvatar.graphql"
)]
#[cfg(feature="CreateAvatar")]
pub struct CreateAvatar;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateStarter.graphql"
)]
#[cfg(feature="CreateStarter")]
pub struct CreateStarter;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateTrade.graphql"
)]
#[cfg(feature="CreateTrade")]
pub struct CreateTrade;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateTradeOffer.graphql"
)]
#[cfg(feature="CreateTradeOffer")]
pub struct CreateTradeOffer;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/CreateTrophy.graphql"
)]
#[cfg(feature="CreateTrophy")]
pub struct CreateTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DeleteGuild.graphql"
)]
#[cfg(feature="DeleteGuild")]
pub struct DeleteGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DeleteMe.graphql"
)]
#[cfg(feature="DeleteMe")]
pub struct DeleteMe;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DeleteTrophy.graphql"
)]
#[cfg(feature="DeleteTrophy")]
pub struct DeleteTrophy;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DescendCave.graphql"
)]
#[cfg(feature="DescendCave")]
pub struct DescendCave;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevRandomItem.graphql"
)]
#[cfg(feature="DevRandomItem")]
pub struct DevRandomItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevSavePetItemPos.graphql"
)]
#[cfg(feature="DevSavePetItemPos")]
pub struct DevSavePetItemPos;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DrinkPotion.graphql"
)]
#[cfg(feature="DrinkPotion")]
pub struct DrinkPotion;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/EatFoodItem.graphql"
)]
#[cfg(feature="EatFoodItem")]
pub struct EatFoodItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/EditGuild.graphql"
)]
#[cfg(feature="EditGuild")]
pub struct EditGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/EvolvePet.graphql"
)]
#[cfg(feature="EvolvePet")]
pub struct EvolvePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/FleeBattle.graphql"
)]
#[cfg(feature="FleeBattle")]
pub struct FleeBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ForceCatchPet.graphql"
)]
#[cfg(feature="ForceCatchPet")]
pub struct ForceCatchPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ForgetManyPets.graphql"
)]
#[cfg(feature="ForgetManyPets")]
pub struct ForgetManyPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ForgetPet.graphql"
)]
#[cfg(feature="ForgetPet")]
pub struct ForgetPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/FormGuild.graphql"
)]
#[cfg(feature="FormGuild")]
pub struct FormGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GenDevPet.graphql"
)]
#[cfg(feature="GenDevPet")]
pub struct GenDevPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GigaCheckoutLink.graphql"
)]
#[cfg(feature="GigaCheckoutLink")]
pub struct GigaCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GigaPortalLink.graphql"
)]
#[cfg(feature="GigaPortalLink")]
pub struct GigaPortalLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GiveUpCatching.graphql"
)]
#[cfg(feature="GiveUpCatching")]
pub struct GiveUpCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/HealPets.graphql"
)]
#[cfg(feature="HealPets")]
pub struct HealPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/InviteToGuild.graphql"
)]
#[cfg(feature="InviteToGuild")]
pub struct InviteToGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/LeaveGuild.graphql"
)]
#[cfg(feature="LeaveGuild")]
pub struct LeaveGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/LinkClick.graphql"
)]
#[cfg(feature="LinkClick")]
pub struct LinkClick;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/Logout.graphql"
)]
#[cfg(feature="Logout")]
pub struct Logout;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/NameCaughtPet.graphql"
)]
#[cfg(feature="NameCaughtPet")]
pub struct NameCaughtPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/PetStorageCheckoutLink.graphql"
)]
#[cfg(feature="PetStorageCheckoutLink")]
pub struct PetStorageCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/PickUpQuestItem.graphql"
)]
#[cfg(feature="PickUpQuestItem")]
pub struct PickUpQuestItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/PlayWithPet.graphql"
)]
#[cfg(feature="PlayWithPet")]
pub struct PlayWithPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/QuestInteraction.graphql"
)]
#[cfg(feature="QuestInteraction")]
pub struct QuestInteraction;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ReportUser.graphql"
)]
#[cfg(feature="ReportUser")]
pub struct ReportUser;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SavePetMoves.graphql"
)]
#[cfg(feature="SavePetMoves")]
pub struct SavePetMoves;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ScareAwayPet.graphql"
)]
#[cfg(feature="ScareAwayPet")]
pub struct ScareAwayPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SellAuctionItem.graphql"
)]
#[cfg(feature="SellAuctionItem")]
pub struct SellAuctionItem;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SendWildBattleSequence.graphql"
)]
#[cfg(feature="SendWildBattleSequence")]
pub struct SendWildBattleSequence;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SetGuildRole.graphql"
)]
#[cfg(feature="SetGuildRole")]
pub struct SetGuildRole;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StabilizePlasma.graphql"
)]
#[cfg(feature="StabilizePlasma")]
pub struct StabilizePlasma;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartCatching.graphql"
)]
#[cfg(feature="StartCatching")]
pub struct StartCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartGuildBattle.graphql"
)]
#[cfg(feature="StartGuildBattle")]
pub struct StartGuildBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartNpcBattle.graphql"
)]
#[cfg(feature="StartNpcBattle")]
pub struct StartNpcBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/StartWildBattle.graphql"
)]
#[cfg(feature="StartWildBattle")]
pub struct StartWildBattle;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/SwapDeadPet.graphql"
)]
#[cfg(feature="SwapDeadPet")]
pub struct SwapDeadPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/TakeIceCream.graphql"
)]
#[cfg(feature="TakeIceCream")]
pub struct TakeIceCream;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/Teleport.graphql"
)]
#[cfg(feature="Teleport")]
pub struct Teleport;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ThrowItemAtPet.graphql"
)]
#[cfg(feature="ThrowItemAtPet")]
pub struct ThrowItemAtPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/TryToFriendCatching.graphql"
)]
#[cfg(feature="TryToFriendCatching")]
pub struct TryToFriendCatching;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/TurnInQuest.graphql"
)]
#[cfg(feature="TurnInQuest")]
pub struct TurnInQuest;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateAvatar.graphql"
)]
#[cfg(feature="UpdateAvatar")]
pub struct UpdateAvatar;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateMainPets.graphql"
)]
#[cfg(feature="UpdateMainPets")]
pub struct UpdateMainPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateMe.graphql"
)]
#[cfg(feature="UpdateMe")]
pub struct UpdateMe;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateMyLocation.graphql"
)]
#[cfg(feature="UpdateMyLocation")]
pub struct UpdateMyLocation;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdatePet.graphql"
)]
#[cfg(feature="UpdatePet")]
pub struct UpdatePet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdatePetItems.graphql"
)]
#[cfg(feature="UpdatePetItems")]
pub struct UpdatePetItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UpdateRep.graphql"
)]
#[cfg(feature="UpdateRep")]
pub struct UpdateRep;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/UsernameChangeCheckoutLink.graphql"
)]
#[cfg(feature="UsernameChangeCheckoutLink")]
pub struct UsernameChangeCheckoutLink;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevAllItems.graphql"
)]
#[cfg(feature="DevAllItems")]
pub struct DevAllItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevGetPetItemMap.graphql"
)]
#[cfg(feature="DevGetPetItemMap")]
pub struct DevGetPetItemMap;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/DevPets.graphql"
)]
#[cfg(feature="DevPets")]
pub struct DevPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetAuctionItems.graphql"
)]
#[cfg(feature="GetAuctionItems")]
pub struct GetAuctionItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetAvatarItems.graphql"
)]
#[cfg(feature="GetAvatarItems")]
pub struct GetAvatarItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetBondLeaderboard.graphql"
)]
#[cfg(feature="GetBondLeaderboard")]
pub struct GetBondLeaderboard;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetDailyQuests.graphql"
)]
#[cfg(feature="GetDailyQuests")]
pub struct GetDailyQuests;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetGigaStoreItems.graphql"
)]
#[cfg(feature="GetGigaStoreItems")]
pub struct GetGigaStoreItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetGuild.graphql"
)]
#[cfg(feature="GetGuild")]
pub struct GetGuild;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetGuildInvites.graphql"
)]
#[cfg(feature="GetGuildInvites")]
pub struct GetGuildInvites;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetInventory.graphql"
)]
#[cfg(feature="GetInventory")]
pub struct GetInventory;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetLinkAnalytics.graphql"
)]
#[cfg(feature="GetLinkAnalytics")]
pub struct GetLinkAnalytics;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetMyLocation.graphql"
)]
#[cfg(feature="GetMyLocation")]
pub struct GetMyLocation;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetNotifs.graphql"
)]
#[cfg(feature="GetNotifs")]
pub struct GetNotifs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetNumUnreadNotifs.graphql"
)]
#[cfg(feature="GetNumUnreadNotifs")]
pub struct GetNumUnreadNotifs;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPet.graphql"
)]
#[cfg(feature="GetPet")]
pub struct GetPet;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPetItems.graphql"
)]
#[cfg(feature="GetPetItems")]
pub struct GetPetItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPetLog.graphql"
)]
#[cfg(feature="GetPetLog")]
pub struct GetPetLog;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPetMoves.graphql"
)]
#[cfg(feature="GetPetMoves")]
pub struct GetPetMoves;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetPets.graphql"
)]
#[cfg(feature="GetPets")]
pub struct GetPets;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetStarterAvatarItems.graphql"
)]
#[cfg(feature="GetStarterAvatarItems")]
pub struct GetStarterAvatarItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetTradeBoxes.graphql"
)]
#[cfg(feature="GetTradeBoxes")]
pub struct GetTradeBoxes;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetTradeOffers.graphql"
)]
#[cfg(feature="GetTradeOffers")]
pub struct GetTradeOffers;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetTrophies.graphql"
)]
#[cfg(feature="GetTrophies")]
pub struct GetTrophies;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetUser.graphql"
)]
#[cfg(feature="GetUser")]
pub struct GetUser;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/GetVendorItems.graphql"
)]
#[cfg(feature="GetVendorItems")]
pub struct GetVendorItems;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/ItemNameSearch.graphql"
)]
#[cfg(feature="ItemNameSearch")]
pub struct ItemNameSearch;
#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/schema.graphql",
	query_path = "src/queries/queries/Me2.graphql"
)]
#[cfg(feature="Me2")]
pub struct Me2;